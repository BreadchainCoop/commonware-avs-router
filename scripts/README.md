# Integration Test Scripts

This directory contains scripts for running integration tests of the BLS signature aggregation system.

## Overview

The integration test validates the complete end-to-end flow:

1. **Local Blockchain Setup**: Starts a local Ethereum blockchain with deployed EigenLayer contracts
2. **BLS Signature Aggregation**: Runs the orchestrator and 3 contributors 
3. **Verification**: Confirms that the counter contract was incremented at least twice through successful signature aggregation

## Files

- `run_integration_test.sh` - Main integration test script
- `verify_increments.rs` - Rust script that monitors and verifies counter increments  
- `Cargo.toml` - Dependencies for the verification script

## Running the Test Locally

### Prerequisites

- Docker and Docker Compose
- Rust toolchain
- All submodules initialized (`git submodule update --init --recursive`)

### Run the Test

```bash
# From the project root
./scripts/run_integration_test.sh
```

The script will:
1. Build the router and node projects
2. Set up environment files for local mode
3. Start Docker containers with local blockchain
4. Start 3 contributors and 1 orchestrator
5. Wait for signature aggregation cycles
6. Verify the counter was incremented at least twice
7. Clean up all processes and containers

### Expected Output

```
✅ SUCCESS: Counter was incremented 2 times (target: 2)
Total time elapsed: 95.3 seconds
✅ Integration test PASSED! Counter was incremented successfully.
```

## CI/CD Integration

The test is also automated through GitHub Actions in `.github/workflows/integration-test.yml`. The CI pipeline:

- Triggers on pushes to `main` and `local-ci` branches
- Runs on Ubuntu with Docker support
- Has a 15-minute timeout
- Provides detailed logs on failure

## How It Works

### System Architecture

The BLS signature aggregation system works as follows:

1. **Orchestrator**: Coordinates the aggregation process
   - Every 30 seconds, creates a new round with payload from counter contract
   - Broadcasts round to all contributors
   - Collects and validates signatures
   - When threshold (3 signatures) is reached, aggregates and executes on-chain

2. **Contributors**: Sign round messages
   - Receive round broadcasts from orchestrator
   - Validate round payload
   - Sign with BLS private key
   - Send signature back to orchestrator

3. **Counter Contract**: Target for on-chain execution
   - `number()` function returns current counter value
   - `increment()` function increments counter (called by orchestrator)

### Test Flow

1. **Setup Phase** (~2 minutes):
   - Start local blockchain with EigenLayer contracts
   - Deploy counter contract and register 3 test operators
   - Start contributor nodes and orchestrator

2. **Aggregation Phase** (~2-3 minutes):
   - Wait for at least 2 signature aggregation cycles
   - Each cycle: round creation → signing → aggregation → increment
   - Monitor counter value every 10 seconds

3. **Verification Phase** (~10 seconds):
   - Check that counter increased by at least 2
   - Success if target increments achieved within timeout

## Troubleshooting

### Common Issues

1. **Docker containers fail to start**
   - Check if ports 8545, 3333, 3334 are available
   - Ensure Docker daemon is running

2. **Contract deployment timeout**
   - Increase timeout in the script
   - Check Docker logs: `docker compose logs`

3. **Contributors fail to connect**
   - Verify keyfiles exist in `eigenlayer-bls-local/.nodes/operator_keys/`
   - Check network connectivity between processes

4. **Signature threshold not reached**
   - Ensure all 3 contributors are running
   - Check contributor logs for errors
   - Verify operator registration in EigenLayer contracts

### Debug Information

The script creates detailed logs in the `logs/` directory:
- `orchestrator.log` - Main orchestrator output
- `contributor1.log`, `contributor2.log`, `contributor3.log` - Individual contributor logs

On test failure, recent log excerpts are displayed automatically.

### Manual Verification

You can also run the verification script separately:

```bash
# Start the system manually (follow README steps)
# Then run verification
cd scripts
source ../.env
cargo run --bin verify_increments
```

## Configuration

### Environment Variables

The test uses these key environment variables:
- `HTTP_RPC` - Blockchain RPC endpoint (http://localhost:8545 for local)
- `AVS_DEPLOYMENT_PATH` - Path to contract deployment JSON
- `CONTRIBUTOR_X_KEYFILE` - BLS private key files for contributors
- `PRIVATE_KEY` - ECDSA private key for transactions

### Timeouts

- Contract deployment: 5 minutes
- Signature aggregation: 2.5 minutes  
- Process initialization: 30-60 seconds per component

### Thresholds

- Target increments: 2 (configurable in verification script)
- Signature threshold: 3 contributors (hardcoded in system)
- Aggregation frequency: 30 seconds (configurable in orchestrator) 