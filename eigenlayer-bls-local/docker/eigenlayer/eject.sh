#!/bin/sh
if [ -z "$REGISTRY_COORDINATOR_ADDRESS" ]; then
  echo "Error: REGISTRY_COORDINATOR_ADDRESS is not set in the environment variables."
  exit 1
fi
if [ -z "$RPC_URL" ]; then
  echo "Error: RPC_URL is not set in the environment variables."
  exit 1
fi


function eject_operator() {
    operatorID=$1
    operator=$(cast call ${REGISTRY_COORDINATOR_ADDRESS} "function getOperatorFromId(bytes32)" ${operatorID} -r ${RPC_URL} | cast parse-bytes32-address)
    echo "Ejecting operator ${operator}"
    if [ $? -ne 0 ]; then
        echo "Error: Failed to get the operator address."
        exit 1
    fi
    cast send ${REGISTRY_COORDINATOR_ADDRESS} "ejectOperator(address,bytes)" ${operator} "0x00" --from ${ejector} --unlocked -r ${RPC_URL} > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Error: Failed to eject the operator."
        exit 1
    fi
}

block=$(cast block-number -r ${RPC_URL})
if [ $? -ne 0 ]; then
  echo -e "${RED}ERROR${RESET} getting the block number"
  exit 1
fi
indexRegistry=$(cast call ${REGISTRY_COORDINATOR_ADDRESS} "function indexRegistry()" -r ${RPC_URL} | cast parse-bytes32-address)
if [ $? -ne 0 ]; then
    echo "Error: Failed to get the index registry address."
    exit 1
fi
operatorsIDRaw=$(cast call ${indexRegistry} "function getOperatorListAtBlockNumber(uint8,uint32)" 0 ${block} -r ${RPC_URL})
if [ $? -ne 0 ]; then
  echo -e "${RED}ERROR${RESET} getting the operator list"
  exit 1
fi
operatorsID=$(cast abi-decode "function(uint8,uint32) returns (bytes32[])" ${operatorsIDRaw})
operatorsID=$(echo ${operatorsID} | tr -d '[' | tr -d ']' | tr -d ',')

ejector=$(cast call ${REGISTRY_COORDINATOR_ADDRESS} "function ejector()" -r ${RPC_URL} | cast parse-bytes32-address)
if [ $? -ne 0 ]; then
    echo "Error: Failed to get the ejector address."
    exit 1
fi
cast rpc anvil_setBalance ${ejector} 0x10000000000000000 -r ${RPC_URL} > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error: Failed to set the ejector balance."
    exit 1
fi
cast rpc anvil_impersonateAccount ${ejector} -r ${RPC_URL} > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Error: Failed to impersonate the ejector."
    exit 1
fi
operatorsIDLength=$(echo ${operatorsID} | wc -w)
countervar=1
for operatorID in ${operatorsID}; do
  eject_operator ${operatorID} 
  echo " ejecting operator $countervar out of $operatorsIDLength"
  countervar=$((countervar + 1))
done
wait
