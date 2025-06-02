#!/bin/sh

if [ -z "$LST_CONTRACT_ADDRESS" ]; then
  echo "Error: LST_CONTRACT_ADDRESS is not set in the environment variables."
  exit 1
fi
if [ -z "$DELEGATION_MANAGER_ADDRESS" ]; then
  echo "Error: DELEGATION_MANAGER_ADDRESS is not set in the environment variables."
  exit 1
fi
if [ -z "$LST_STRATEGY_ADDRESS" ]; then
  echo "Error: LST_STRATEGY_ADDRESS is not set in the environment variables."
  exit 1
fi
if [ -z "$STRATEGY_MANAGER_ADDRESS" ]; then
  echo "Error: STRATEGY_MANAGER_ADDRESS is not set in the environment variables."
  exit 1
fi
if [ -z "$RPC_URL" ]; then
  echo "Error: RPC_URL is not set in the environment variables."
  exit 1
fi

if [ "$ENVIRONMENT" = "TESTNET" ]; then
  if [ -z "$FUNDED_KEY" ]; then
    echo "Error: FUNDED_KEY is not set in the environment variables. This is required for testnet."
    exit 1
  fi
fi
sleep 10

rm -rf $HOME/.nodes/operator_keys/*

if [ -n "$TEST_ACCOUNTS" ]; then
    num_accounts=$TEST_ACCOUNTS
else
    num_accounts=1
fi

for i in $(seq 1 $num_accounts); do
    echo "Creating test account $i of $num_accounts"
    ./register.sh
    if [ $? -ne 0 ]; then
        echo "Error: Failed to create test account $i"
        exit 1
    fi
done

if [ "$ENVIRONMENT" = "TESTNET" ]; then
    echo "Sleeping for 5 minutes to allow allocation delay to be processed on testnet..."
    sleep 360
fi

# deploy script 
# Create deployer account and fund it
DEPLOYER_INFO=$(cast wallet new --json)
DEPLOYER_KEY=$(echo "$DEPLOYER_INFO" | jq -r '.[0].private_key')
DEPLOYER_ADDRESS=$(echo "$DEPLOYER_INFO" | jq -r '.[0].address')

if [ "$ENVIRONMENT" = "TESTNET" ]; then
    DEPLOYER_KEY=$FUNDED_KEY
else
    cast rpc anvil_setBalance $DEPLOYER_ADDRESS 0x10000000000000000000 --rpc-url $RPC_URL > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Error: Failed to set balance for deployer account"
        exit 1
    fi
fi

export PRIVATE_KEY=$DEPLOYER_KEY
chain_id=$(cast chain-id --rpc-url $RPC_URL)
cd bls-middleware/contracts && forge script script/IncredibleSquaringDeployer.s.sol --rpc-url $RPC_URL --skip src/libraries/BN256G2.sol --optimize --broadcast --private-key $PRIVATE_KEY > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error: Failed to run middleware deployment script"
fi
cp script/deployments/incredible-squaring/$chain_id.json ~/.nodes/avs_deploy.json
cp script/deployments/incredible-squaring/$chain_id.json avs_deploy.json
forge script script/UAMPermissions.s.sol --rpc-url $RPC_URL --broadcast --private-key $PRIVATE_KEY > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error: Failed to run UAMPermissions script"
fi
forge script script/SetupMiddleware.s.sol --rpc-url $RPC_URL --broadcast --private-key $PRIVATE_KEY > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error: Failed to run SetupMiddleware script"
fi

# Get stake registry address once
STAKE_REGISTRY=$(cat avs_deploy.json | jq -r '.addresses.stakeRegistry')
if [ -z "$STAKE_REGISTRY" ] || [ "$STAKE_REGISTRY" = "null" ]; then
    echo "Error: Failed to get stake registry address from deployment JSON"
    exit 1
fi

for i in $(seq 1 $num_accounts); do
    echo "Processing operator $i..."
    
    # Copy operator keys
    cp ~/.nodes/operator_keys/testacc${i}.private.ecdsa.key.json private.ecdsa.json
    cp ~/.nodes/operator_keys/testacc${i}.private.bls.key.json private.bls.json
    
    # Get operator private key and address
    export OPERATOR_PRIVATE_KEY=$(cat private.ecdsa.json | jq -r .privateKey)
    if [ -z "$OPERATOR_PRIVATE_KEY" ]; then
        echo "Error: Failed to extract private key from private.ecdsa.json for operator $i"
        exit 1
    fi
    
    OPERATOR_ADDRESS=$(cast wallet address --private-key $OPERATOR_PRIVATE_KEY)
    
    if [ "$ENVIRONMENT" = "local" ]; then
        echo "Getting delegation manager and overriding allocation delay..."
        
        # Set balance and impersonate the delegation manager
        cast rpc anvil_setBalance $DELEGATION_MANAGER_ADDRESS 0x10000000000000000 --rpc-url $RPC_URL > /dev/null 2>&1
        if [ $? -ne 0 ]; then
            echo "Error: Failed to set balance for delegation manager"
            exit 1
        fi
        
        cast rpc anvil_impersonateAccount $DELEGATION_MANAGER_ADDRESS --rpc-url $RPC_URL > /dev/null 2>&1
        if [ $? -ne 0 ]; then
            echo "Error: Failed to impersonate delegation manager"
            exit 1
        fi
        
        # Call the function to override allocation delay
        cast send $ALLOCATION_MANAGER_ADDRESS "setAllocationDelay(address,uint32)" $OPERATOR_ADDRESS 0 --from $DELEGATION_MANAGER_ADDRESS --unlocked --rpc-url $RPC_URL > /dev/null 2>&1
        if [ $? -ne 0 ]; then
            echo "Error: Failed to override allocation delay"
            exit 1
        fi
    fi
    # Set the operator ID for registration
    export OPERATOR_ID="testacc${i}"
    
    forge script script/RegisterOperator.s.sol --rpc-url $RPC_URL --broadcast --private-key $OPERATOR_PRIVATE_KEY --isolate --slow --skip-simulation #> /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Error: Failed to register operator $OPERATOR_ADDRESS"
    fi
    
    WEIGHT=$(cast call $STAKE_REGISTRY "weightOfOperatorForQuorum(uint8,address)(uint96)" 0 $OPERATOR_ADDRESS --rpc-url $RPC_URL)
    if [ $? -ne 0 ]; then
        echo "Error: Failed to get operator weight for quorum for operator $i"
        exit 1
    fi
    
    echo "Operator $i weight in quorum 0: $WEIGHT"
done

# Keep container open for debugging
echo "Script execution finished. Keeping container open..."
while true; do sleep 1; done
