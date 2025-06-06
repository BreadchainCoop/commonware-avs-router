## Set Up Enviroment 

```sh
git submodule update --init --recursive
cd eigenlayer-bls-local
```
Follow the instructions in the `eigenlayer-bls-local/README.md` to run TESTNET mode 

When the process is finished (You should see `Operator X weight in quorum 0: 11887896997963931 [1.188e16]` where X is the number of test accounts configured) 

```sh
docker compose down 
```


## Set Up Nodes 
```sh
cd ../commonware-avs-node
cargo build 
cp example.env .env 
```
Fill in the following env variables in the commonware-avs-node 
```
COUNTER_ADDRESS
REGISTRY_COORDINATOR_ADDRESS
BLS_APK_REGISTRY_ADDRESS
```
from the `eigenlayer-bls-local/.nodes/avs_deploy.json` file

You can do this manually or use the update_env.sh script

```
cd ..
chmod +x update_node_env.sh
./update_node_env.sh
```

Manually add your private key and rpc url to the env

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
cp example.env .env 
```
Populate env variables as required 

You can do this manually or use the update_router_env.sh script:

```sh
chmod +x update_router_env.sh
./update_router_env.sh
```

Manually add your private key and rpc url to the env

## Orchestrator
```bash
cargo run --release -- --key-file keys/orchestrator.json --port 3000 
```
