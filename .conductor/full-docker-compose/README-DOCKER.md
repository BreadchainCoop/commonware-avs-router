# Docker Compose Environment for AVS Router

## Current Status

The docker-compose environment provides the core infrastructure services needed for AVS development:

### Working Services

1. **Ethereum (Anvil)** - Local blockchain on port 8545
2. **EigenLayer** - Deploys contracts and registers operators
3. **Signer (Cerberus)** - BLS signing service on ports 9081 and 50051

### Known Issues

The AVS Router and Node services currently face compatibility issues when running in Docker:

1. **Key Format Mismatch**: The Docker images expect BLS keys with G2 point coordinates (`g2_x1`, `g2_x2`, `g2_y1`, `g2_y2`), but EigenLayer generates different key formats.

2. **Platform Architecture**: The images are built for linux/amd64 but may have issues on ARM64 (Apple Silicon) systems due to emulation.

3. **Permission Issues**: File permission errors occur when containers try to read mounted volumes, particularly on ARM64 systems.

## Setup Instructions

### 1. Start Infrastructure Services

```bash
# Copy environment file
cp example.env .env

# Start services
docker compose up -d

# Check status
docker compose ps
```

### 2. Wait for EigenLayer Setup

The EigenLayer container will:
- Deploy contracts
- Register test operators
- Generate BLS keys
- Create `avs_deploy.json`

Check progress:
```bash
docker compose logs eigenlayer --tail=20
```

Wait for "Contract deployment and run complete" message.

### 3. Running AVS Services Natively (Recommended)

Due to the Docker compatibility issues, run the AVS router and nodes natively:

```bash
# Build the router
cargo build --release

# Build the nodes
cd commonware-avs-node
cargo build --release
cd ..

# Run router
./target/release/commonware-avs-router --key-file config/orchestrator.json --port 3000

# Run nodes (in separate terminals)
cd commonware-avs-node
./target/release/commonware-avs-node --key-file ../.nodes/operator_keys/testacc1.private.bls.key.json --port 3001 --orchestrator orchestrator.json
./target/release/commonware-avs-node --key-file ../.nodes/operator_keys/testacc2.private.bls.key.json --port 3002 --orchestrator orchestrator.json
./target/release/commonware-avs-node --key-file ../.nodes/operator_keys/testacc3.private.bls.key.json --port 3003 --orchestrator orchestrator.json
```

## Files Generated

After EigenLayer setup completes:

- `.nodes/avs_deploy.json` - Contract deployment addresses
- `.nodes/operator_keys/testacc*.bls.key.json` - Encrypted BLS keys
- `.nodes/operator_keys/testacc*.private.bls.key.json` - Private BLS keys
- `.nodes/operator_keys/testacc*.ecdsa.key.json` - ECDSA keys

## Troubleshooting

### Permission Denied Errors

```bash
chmod -R 777 .nodes/
chmod -R 755 config/
```

### Check Service Logs

```bash
docker compose logs ethereum --tail=20
docker compose logs eigenlayer --tail=20
docker compose logs signer --tail=20
```

### Reset Environment

```bash
docker compose down -v
rm -rf .nodes/
docker compose up -d
```

## Future Improvements

1. Build multi-architecture Docker images (linux/amd64 and linux/arm64)
2. Standardize key format between EigenLayer and AVS services
3. Fix volume permission issues in Docker containers
4. Add health checks for all services
5. Create initialization scripts to handle key conversion