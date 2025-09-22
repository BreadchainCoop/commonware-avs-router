# Commonware AVS Router

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ghcr.io/breadchaincoop/commonware--avs--router-blue.svg)](https://github.com/BreadchainCoop/commonware-avs-router/pkgs/container/commonware-avs-router)

A BLS signature aggregation protocol with onchain execution for EigenLayer AVS operators.

## Overview

The router coordinates multiple operators to sign messages, aggregates their signatures when a threshold is reached, and executes the result onchain.

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Rust
- Git

### Local Development
1. **Configure environment:**
```bash
cp example.env .env

# Edit .env to set your configuration

cp config/config.example.json config/config.json

# Edit "config/config.json" if you need different operator socket addresses
```

2. **Start services:**
```bash
docker compose up

# Add -d flag to run in background
```

### Manual Node Setup
If you need to run nodes outside of Docker, you can use the following process.

1. **Configure environment:**
```bash
cd commonware-avs-node

cp example.env .env

# Edit .env to set your configuration
```

2. **Build binaries:**
```bash
cargo build --release
```

3. **Run nodes (one per terminal):**
```bash
# Node 1
cargo run --release -- --key-file $CONTRIBUTOR_1_KEYFILE --port 3001 --orchestrator orchestrator.json

# Node 2
cargo run --release -- --key-file $CONTRIBUTOR_2_KEYFILE --port 3002 --orchestrator orchestrator.json

# Node 3
cargo run --release -- --key-file $CONTRIBUTOR_3_KEYFILE --port 3003 --orchestrator orchestrator.json
```

### Manual Router Setup

If you need to run a router outside of Docker, you can use the following process.

1. **Configure environment:**
```bash
cp example.env .env

# Edit .env to set your configuration

cp config/config.example.json config/config.json

# Edit "config/config.json" if you need different operator socket addresses
```

2. **Build binaries:**
```bash
cargo build --release
```

3. **Start router:**
```bash
# Orchestrator mode
cargo run --release -- --key-file config/orchestrator.json --port 3000

# Or with ingress enabled
cargo run --release -- --ingress
```

## Architecture

The system consists of:

- **Orchestrator**: Coordinates the aggregation process
- **Creator**: Generates payloads and manages rounds  
- **Executor**: Handles onchain execution
- **Validator**: Validates messages and signatures
- **Contributors**: Operator nodes that sign messages (implemented in [`commonware-avs-node`](https://github.com/BreadchainCoop/commonware-avs-node) submodule)

### Usecases

The router supports multiple usecases for different onchain operations:

- **[Counter Usecase](src/usecases/counter/README.md)**: Simple counter increment with BLS signature aggregation
- More usecases can be added by implementing the `Creator` and `Executor` traits

See individual usecase READMEs for detailed architecture diagrams and implementation details.

## Configuration

### Environment Variables

Required environment variables:
- `HTTP_RPC`: HTTP RPC endpoint
- `WS_RPC`: WebSocket RPC endpoint
- `AVS_DEPLOYMENT_PATH`: Path to deployment JSON file
- `CONTRIBUTOR_X_KEYFILE`: BLS key files for contributors
- `PRIVATE_KEY`: Private key for transactions. **NOTE:** Address must be funded on Holesky testnet

Optional environment variables:
- `AGGREGATION_FREQUENCY`: Signature aggregation frequency in seconds, supports fractional values (default: 30)
  - Examples: `30` (30 seconds), `1` (1 second), `0.1` (100ms), `0.5` (500ms)
- `THRESHOLD`: Minimum signatures required for aggregation
- `INGRESS`: Enable HTTP ingress mode (true/false)
- `INGRESS_ADDRESS`: Address for ingress server (default: 0.0.0.0:8080)

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
      - AGGREGATION_FREQUENCY=${AGGREGATION_FREQUENCY:-30}
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
./scripts/router_e2e_local.sh
```
