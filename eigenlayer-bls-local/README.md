# BLS Local 

This repository contains the configuration and setup for running the BLS AVS (Actively Validated Service) infrastructure using Docker Compose.
Relies on [BLS-middleware](https://github.com/BreadchainCoop/bls-middleware)
## Prerequisites

- Docker
- Docker Compose

## Setup

2. Create a `.env` file in the root directory and add the following environment variables:
   ```
   cp example.env .env
   ```
   Make sure you have filled in all the variables 
   ```
    DELEGATION_MANAGER_ADDRESS= # Depends on the desired ENV 
    STRATEGY_MANAGER_ADDRESS=
    REGISTRY_COORDINATOR_ADDRESS=
    LST_CONTRACT_ADDRESS=
    LST_STRATEGY_ADDRESS=
    FORK_URL=
    RPC_URL=http://ethereum:8545
    WEBSOCKET_RPC_URL=ws://ethereum:8545
    ENVIRONMENT= # Depends on the desired ENV 
    MAX_OPERATOR_RETRY_ATTEMPTS=10
    SERVER_PRIVATE_KEY= 
   ```

3. Build and start the services:
   ```
   docker-compose up --build
   ```
Note that the nodes and node selector only start up after the eigenlayer setup container has exited  

## Services

The Docker Compose setup includes the following services:

1. `ethereum`: An Ethereum node for local development and testing.
2. `eigenlayer`: Sets up EigenLayer and registers operators.

## Configuration

- The `docker/eigenlayer/register.sh` script creates test accounts and registers them as operators.
- Node configurations are stored in `.nodes/configs/`.
- Operator keys are stored in `.nodes/operator_keys/`.

### BLS AVS Configuration File (config.json)

The BLS AVS configuration is defined in `docker/eigenlayer/config.json` and controls the key parameters for the AVS:

```json
{
    "quorum": {
        "minimumStake": "1",
        "maxOperatorCount": 32,
        "kickBIPsOfOperatorStake": 10000,
        "kickBIPsOfTotalStake": 100
    },
    "metadata": {
        "uri": "metadataURI"
    },
    "operators": {
        "testacc1": {
            "socketAddress": "127.0.0.1:3000"
        },
        "testacc2": {
            "socketAddress": "127.0.0.1:3000"
        },
        "testacc3": {
            "socketAddress": "127.0.0.1:3000"
        }
    }
}
```

#### Configuration Options

##### Quorum Settings
- `minimumStake`: The minimum amount of stake (in wei) required for an operator to participate in the AVS. This is the minimum amount of tokens an operator must stake to be considered active.
- `maxOperatorCount`: The maximum number of operators allowed in the AVS. Default is 32. This limits the total number of operators that can participate in the service.
- `kickBIPsOfOperatorStake`: The percentage of an operator's stake that can be slashed (in basis points). 10000 basis points = 100%. This determines how much of an operator's stake can be slashed if they misbehave.
- `kickBIPsOfTotalStake`: The percentage of total stake that can be slashed (in basis points). 100 basis points = 1%. This sets the maximum amount of total stake that can be slashed across all operators.

##### Metadata
- `uri`: The URI pointing to the AVS metadata. This should contain information about the AVS service, its purpose, and any relevant documentation.

##### Operators
- Each operator entry contains:
  - `socketAddress`: The network address where the operator's node can be reached. Format should be `IP:PORT`.
  - Multiple operators can be configured, each with their own unique identifier (e.g., "testacc1", "testacc2", etc.)

#### Recommended Settings

For a production environment:
- `minimumStake`: Set based on your security requirements. Higher values ensure more committed operators.
- `maxOperatorCount`: Adjust based on your network's capacity and decentralization goals.
- `kickBIPsOfOperatorStake`: Typically set to 10000 (100%) to allow full slashing of misbehaving operators.
- `kickBIPsOfTotalStake`: Set based on your risk tolerance. Lower values (e.g., 100 = 1%) provide more protection against mass slashing events.

For a test environment:
- You can use lower values for `minimumStake` to make testing easier
- Keep `maxOperatorCount` small (e.g., 3-5) for testing purposes
- Use test operator addresses with appropriate test network configurations

## Accessing the Services

- Ethereum RPC: http://localhost:8545
- Node Selector: http://localhost:8080

## Notes

- The `.gitignore` file is set up to exclude sensitive information like operator keys and test account configs.
- Make sure to keep your `.env` file secure and do not commit it to version control.

For more detailed information about each component, refer to the individual Dockerfile and configuration files in the `docker/` directory.
