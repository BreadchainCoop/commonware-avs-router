## Set Up Enviroment 

```sh
cd eigenlayer-bls-local
docker compose build 
```
Follow the instructions in the `eigenlayer-bls-local/README.md` to run testnet mode 

```sh
docker compose up 
```
When the process is finished (You should see `Operator X weight in quorum 0: 11887896997963931 [1.188e16]` where X is the number of test accounts configured) 

```sh
docker compose down 
```


## Set Up Nodes 
```sh
cd `commonware-avs-node`
cargo build 
cp .example.env .env 
```
Fill in the following env variables 
```
COUNTER_ADDRESS
REGISTRY_COORDINATOR_ADDRESS
BLS_APK_REGISTRY_ADDRESS
```
from the `eigenlayer-bls-local/.nodes/avs_deploy.json` file
(Snippet to do so using `jq`) 
```sh
jq -r '.addresses.registryCoordinator' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/REGISTRY_COORDINATOR_ADDRESS=/' >> commonware-avs-node/.env
jq -r '.addresses.blsapkRegistry' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/BLS_APK_REGISTRY_ADDRESS=/' >> commonware-avs-node/.env
jq -r '.addresses.counter' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/COUNTER_ADDRESS=/' >> commonware-avs-node/.env
```

For `CONTRIBUTOR_X_KEYFILE`, use the  `../eigenlayer-bls-local/.nodes/operator_keys/testaccX.bls.key.json` keyfiles
Populate : 
In 3 different terminals: 


## Contributor 1
```bash
cd commonware-avs-node
source .env
cargo run --release -- --key-file $CONTRIBUTOR_1_KEYFILE --port 3001 --orchestrator orchestrator.json 
```

## Contributor 2
```bash
cd commonware-avs-node
source .env
cargo run --release -- --key-file $CONTRIBUTOR_2_KEYFILE --port 3002 --orchestrator orchestrator.json 

```

## Contributor 3
```bash
cd commonware-avs-node
source .env
cargo run --release -- --key-file $CONTRIBUTOR_3_KEYFILE --port 3003 --orchestrator orchestrator.json 
```
Note that the further in time you get from the deployment, the longer the init for the contributors will take and you may need to init them 1 by 1 in 
order to not max out rpc limits.

# Run Router 

```sh
cp .example.env .env 
```
Populate env variables as required 

(Snippet to do so using `jq` for deployed variables) 
```sh
jq -r '.addresses.registryCoordinator' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/REGISTRY_COORDINATOR_ADDRESS=/' >> .env
jq -r '.addresses.blsapkRegistry' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/BLS_APK_REGISTRY_ADDRESS=/' >> .env
jq -r '.addresses.counter' eigenlayer-bls-local/.nodes/avs_deploy.json | sed 's/^/COUNTER_ADDRESS=/' >> .env
```
## Orchestrator
```bash
cargo run --release -- --key-file keys/orchestrator.json --port 3000 
```
