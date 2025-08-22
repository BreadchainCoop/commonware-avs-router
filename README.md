# Commonware AVS Router

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ghcr.io/breadchaincoop/commonware--avs--router-blue.svg)](https://github.com/BreadchainCoop/commonware-avs-router/pkgs/container/commonware-avs-router)

A BLS signature aggregation protocol with on-chain execution for EigenLayer AVS operators.

## Overview

The router coordinates multiple operators to sign messages, aggregates their signatures when a threshold is reached, and executes the result on-chain.

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Rust
- Git

### Local Development

1. **Start local blockchain environment:**
```bash
git submodule update --init --recursive
cd eigenlayer-bls-local
docker compose up
```
Wait for deployment completion (you'll see operator weights displayed).

2. **Configure environment:**
```bash
cd ../
cp example.env .env
# Edit .env with your private key and local RPC URLs
```

3. **Start contributors (3 terminals):**
```bash
cd commonware-avs-node
cargo build
cp example.env .env
# Edit .env for local mode

# Terminal 1
cargo run --release -- --key-file $CONTRIBUTOR_1_KEYFILE --port 3001 --orchestrator orchestrator.json

# Terminal 2  
cargo run --release -- --key-file $CONTRIBUTOR_2_KEYFILE --port 3002 --orchestrator orchestrator.json

# Terminal 3
cargo run --release -- --key-file $CONTRIBUTOR_3_KEYFILE --port 3003 --orchestrator orchestrator.json
```

4. **Start orchestrator:**
```bash
cd ../
cargo run --release -- --key-file config/orchestrator.json --port 3000
```

## Architecture

The system consists of:

- **Orchestrator**: Coordinates the aggregation process
- **Creator**: Generates payloads and manages rounds  
- **Executor**: Handles on-chain execution
- **Validator**: Validates messages and signatures
- **Contributors**: Operator nodes that sign messages (implemented in [`commonware-avs-node`](https://github.com/BreadchainCoop/commonware-avs-node) submodule)

## Configuration

### Environment Variables

Required environment variables:
- `HTTP_RPC`: HTTP RPC endpoint
- `WS_RPC`: WebSocket RPC endpoint
- `AVS_DEPLOYMENT_PATH`: Path to deployment JSON file
- `PRIVATE_KEY`: Private key for transactions
- `CONTRIBUTOR_X_KEYFILE`: BLS key files for contributors

Contract addresses are automatically loaded from the deployment JSON file.

### Docker

Pull the latest image:
```bash
docker pull ghcr.io/breadchaincoop/commonware-avs-router:latest
```

Run with Docker Compose:
```yaml
version: '3.8'
services:
  orchestrator:
    image: ghcr.io/breadchaincoop/commonware-avs-router:latest
    volumes:
      - ./config:/app/config
      - ./keys:/app/keys
    environment:
      - HTTP_RPC=${HTTP_RPC}
      - WS_RPC=${WS_RPC}
      - AVS_DEPLOYMENT_PATH=/app/config/avs_deploy.json
      - PRIVATE_KEY=${PRIVATE_KEY}
      - CONTRIBUTOR_1_KEYFILE=/app/keys/contributor1.bls.key.json
      - CONTRIBUTOR_2_KEYFILE=/app/keys/contributor2.bls.key.json
      - CONTRIBUTOR_3_KEYFILE=/app/keys/contributor3.bls.key.json
    ports:
      - "3000:3000"
    command: ["--key-file", "/app/config/orchestrator.json", "--port", "3000"]
```

## Ingress Mode

Enable HTTP endpoints for external task requests:

```bash
INGRESS=true cargo run --release -- --key-file config/orchestrator.json --port 3000
```

Trigger tasks via HTTP:
```bash
curl -X POST http://localhost:8080/trigger \
  -H "Content-Type: application/json" \
  -d '{"body": {"var1": "value1", "var2": "value2", "var3": "value3"}}'
```

## Development

### Dependencies
- `alloy`: Ethereum interaction
- `bn254`: BLS signature operations  
- `commonware_cryptography`: Cryptographic operations
- `commonware_p2p`: P2P networking
- `commonware_runtime`: Runtime utilities

### Code Quality
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing

Run unit tests:
```bash
cargo test --lib
```

Run end-to-end tests:
```bash
chmod +x scripts/router_e2e_local.sh
scripts/router_e2e_local.sh
```
