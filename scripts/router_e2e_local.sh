#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_DIR="$PROJECT_ROOT/logs"
PIDS_FILE="$PROJECT_ROOT/test_processes.pids"

# Create logs directory
mkdir -p "$LOG_DIR"

# Cleanup function
cleanup() {
    echo -e "${YELLOW}Cleaning up processes...${NC}"
    
    # Kill processes from PID file
    if [ -f "$PIDS_FILE" ]; then
        while read -r pid_line; do
            if [ -n "$pid_line" ]; then
                pid=$(echo "$pid_line" | cut -d: -f2)
                name=$(echo "$pid_line" | cut -d: -f1)
                if kill -0 "$pid" 2>/dev/null; then
                    echo "Killing $name (PID: $pid)"
                    kill "$pid" || true
                fi
            fi
        done < "$PIDS_FILE"
        rm -f "$PIDS_FILE"
    fi
    
    # Stop Docker containers
    echo "Stopping Docker containers..."
    cd "$PROJECT_ROOT/eigenlayer-bls-local"
    docker compose down || true
    
    echo -e "${GREEN}Cleanup completed${NC}"
}

# Set trap for cleanup
trap cleanup EXIT INT TERM

echo -e "${GREEN}Starting BLS Signature Aggregation Integration Test${NC}"
echo "Project root: $PROJECT_ROOT"
echo "Logs directory: $LOG_DIR"

# Ask for private key at the beginning
echo -e "${YELLOW}Configuration Setup${NC}"

# Check if private key already exists in .env files
EXISTING_PRIVATE_KEY=""

# Check main router .env first
if [ -f ".env" ]; then
    EXISTING_PRIVATE_KEY=$(grep "^PRIVATE_KEY=" .env 2>/dev/null | cut -d'=' -f2)
fi

# If not found in main .env, check eigenlayer-bls-local .env for PRIVATE_KEY
if [ -z "$EXISTING_PRIVATE_KEY" ] && [ -f "eigenlayer-bls-local/.env" ]; then
    EXISTING_PRIVATE_KEY=$(grep "^PRIVATE_KEY=" eigenlayer-bls-local/.env 2>/dev/null | cut -d'=' -f2)
fi

# If still not found, check eigenlayer-bls-local .env for FUNDED_KEY
if [ -z "$EXISTING_PRIVATE_KEY" ] && [ -f "eigenlayer-bls-local/.env" ]; then
    EXISTING_PRIVATE_KEY=$(grep "^FUNDED_KEY=" eigenlayer-bls-local/.env 2>/dev/null | cut -d'=' -f2)
fi

# Check if we found a valid private key (not empty and not the default example)
if [ -n "$EXISTING_PRIVATE_KEY" ] && [ "$EXISTING_PRIVATE_KEY" != "" ] && [ "$EXISTING_PRIVATE_KEY" != "0xba35a33f95b443f059e794e4f440d22021400267fff05df4f4d3d2d8eee07215" ]; then
    echo "Found existing private key: ${EXISTING_PRIVATE_KEY:0:10}...${EXISTING_PRIVATE_KEY: -4}"
    echo -n "Press Enter to keep existing key, or enter new private key: "
    read -r PRIVATE_KEY
    if [ -z "$PRIVATE_KEY" ]; then
        PRIVATE_KEY="$EXISTING_PRIVATE_KEY"
        echo "Using existing private key"
    fi
else
    echo -n "Enter your private key (must be funded on Holesky): "
    read -r PRIVATE_KEY
    if [ -z "$PRIVATE_KEY" ]; then
        echo -e "${RED}Error: Private key is required${NC}"
        exit 1
    fi
fi

# Step 1: Build projects
echo -e "${YELLOW}Step 1: Building projects...${NC}"
cd "$PROJECT_ROOT"
echo "Building router..."
cargo build --release --quiet
cd commonware-avs-node
echo "Building AVS node..."
cargo build --release --quiet
cd "$PROJECT_ROOT"

# Step 2: Set up environment files
echo -e "${YELLOW}Step 2: Setting up environment files...${NC}"
cp example.env .env
cp commonware-avs-node/example.env commonware-avs-node/.env

# Update main .env file for local mode
echo "Configuring main .env for local mode..."
# Replace RPC URLs
sed -i '' 's|^HTTP_RPC=.*|HTTP_RPC=http://localhost:8545|' .env
sed -i '' 's|^WS_RPC=.*|WS_RPC=ws://localhost:8545|' .env
# Set the private key
sed -i '' "s|^PRIVATE_KEY=.*|PRIVATE_KEY=$PRIVATE_KEY|" .env

# Update commonware-avs-node .env file for local mode
echo "Configuring commonware-avs-node .env for local mode..."
sed -i '' 's|^HTTP_RPC=.*|HTTP_RPC=http://localhost:8545|' commonware-avs-node/.env
sed -i '' 's|^WS_RPC=.*|WS_RPC=ws://localhost:8545|' commonware-avs-node/.env
sed -i '' 's|^AVS_DEPLOYMENT_PATH=.*|AVS_DEPLOYMENT_PATH="../eigenlayer-bls-local/.nodes/avs_deploy.json"|' commonware-avs-node/.env
# Set the private key
sed -i '' "s|^PRIVATE_KEY=.*|PRIVATE_KEY=$PRIVATE_KEY|" commonware-avs-node/.env

# Step 3: Start local blockchain environment
echo -e "${YELLOW}Step 3: Setting up local blockchain environment...${NC}"
cd eigenlayer-bls-local

# Copy and configure eigenlayer-bls-local .env properly
echo "Configuring eigenlayer-bls-local .env..."
cp example.env .env

# Set required environment variables for LOCAL mode
echo "Setting LOCAL mode configuration..."
sed -i '' 's|^ENVIRONMENT=.*|ENVIRONMENT=LOCAL|' .env
sed -i '' 's|^RPC_URL=.*|RPC_URL=http://ethereum:8545|' .env

# Set FORK_URL to Holesky RPC for forking (required for LOCAL mode)
sed -i '' 's|^FORK_URL=.*|FORK_URL=https://ethereum-holesky.publicnode.com|' .env

# Uncomment Holesky contract addresses for LOCAL mode
sed -i '' 's|^#DELEGATION_MANAGER_ADDRESS=0xA44151489861Fe9e3055d95adC98FbD462B948e7|DELEGATION_MANAGER_ADDRESS=0xA44151489861Fe9e3055d95adC98FbD462B948e7|' .env
sed -i '' 's|^#STRATEGY_MANAGER_ADDRESS=0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6|STRATEGY_MANAGER_ADDRESS=0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6|' .env
sed -i '' 's|^#LST_CONTRACT_ADDRESS=0x3F1c547b21f65e10480dE3ad8E19fAAC46C95034|LST_CONTRACT_ADDRESS=0x3F1c547b21f65e10480dE3ad8E19fAAC46C95034|' .env
sed -i '' 's|^#LST_STRATEGY_ADDRESS=0x7D704507b76571a51d9caE8AdDAbBFd0ba0e63d3|LST_STRATEGY_ADDRESS=0x7D704507b76571a51d9caE8AdDAbBFd0ba0e63d3|' .env
sed -i '' 's|^#BLS_SIGNATURE_CHECKER_ADDRESS=0xca249215e082e17c12bb3c4881839a3f883e5c6b|BLS_SIGNATURE_CHECKER_ADDRESS=0xca249215e082e17c12bb3c4881839a3f883e5c6b|' .env
sed -i '' 's|^#OPERATOR_STATE_RETRIEVER_ADDRESS=0xB4baAfee917fb4449f5ec64804217bccE9f46C67|OPERATOR_STATE_RETRIEVER_ADDRESS=0xB4baAfee917fb4449f5ec64804217bccE9f46C67|' .env
sed -i '' 's|^#ALLOCATION_MANAGER_ADDRESS=0x78469728304326CBc65f8f95FA756B0B73164462|ALLOCATION_MANAGER_ADDRESS=0x78469728304326CBc65f8f95FA756B0B73164462|' .env

# Ensure required service configuration variables are set
if ! grep -q "^CERBERUS_GRPC_PORT=" .env; then
    echo "CERBERUS_GRPC_PORT=50051" >> .env
else
    sed -i '' 's|^CERBERUS_GRPC_PORT=.*|CERBERUS_GRPC_PORT=50051|' .env
fi

if ! grep -q "^CERBERUS_METRICS_PORT=" .env; then
    echo "CERBERUS_METRICS_PORT=9081" >> .env
else
    sed -i '' 's|^CERBERUS_METRICS_PORT=.*|CERBERUS_METRICS_PORT=9081|' .env
fi

if ! grep -q "^SIGNER_ENDPOINT=" .env; then
    echo "SIGNER_ENDPOINT=http://signer:50051" >> .env
else
    sed -i '' 's|^SIGNER_ENDPOINT=.*|SIGNER_ENDPOINT=http://signer:50051|' .env
fi

# Ensure TEST_ACCOUNTS is set
if ! grep -q "^TEST_ACCOUNTS=" .env; then
    echo "TEST_ACCOUNTS=3" >> .env
else
    sed -i '' 's|^TEST_ACCOUNTS=.*|TEST_ACCOUNTS=3|' .env
fi

# Set the private key and funded key for eigenlayer-bls-local
sed -i '' "s|^PRIVATE_KEY=.*|PRIVATE_KEY=$PRIVATE_KEY|" .env
sed -i '' "s|^FUNDED_KEY=.*|FUNDED_KEY=$PRIVATE_KEY|" .env

echo "Final eigenlayer-bls-local .env configuration:"
cat .env

# Build the Docker images first (as per README instructions)
echo -e "${YELLOW}Building Docker images...${NC}"
echo -n "Do you want to build with --no-cache? Ensures fresh images. (y/N): "
read -r use_no_cache

if [[ "$use_no_cache" =~ ^[Yy]$ ]]; then
    echo "Building with --no-cache (this may take a while)..."
    docker compose build --no-cache
else
    echo "Building with cache..."
    docker compose build
fi

# Start the Docker containers
echo -e "${YELLOW}Starting Docker containers...${NC}"
docker compose up -d

# Wait for services to be ready
echo "Waiting for ethereum service to be ready..."
sleep 10

# Wait for the eigenlayer service to complete its setup
echo "Waiting for eigenlayer setup to complete..."
timeout=600  # Increased timeout for build and deployment
elapsed=0
while [ $elapsed -lt $timeout ]; do
    # Check if deployment has completed by looking for completion message in logs
    if docker compose logs eigenlayer 2>/dev/null | grep -q "Script execution finished. Keeping container open..." && [ -f .nodes/avs_deploy.json ] && [ -s .nodes/avs_deploy.json ]; then
        echo -e "${GREEN}EigenLayer setup has completed successfully${NC}"
        break
    fi
    
    # Check for any error indicators
    error_check=$(docker compose logs eigenlayer 2>/dev/null | grep -i "error\|failed" | tail -1 || echo "")
    if [ -n "$error_check" ]; then
        echo -e "${YELLOW}Detected potential error: $error_check${NC}"
    fi
    
    echo "Waiting for eigenlayer setup to complete... ($elapsed/$timeout seconds)"
    sleep 15
    elapsed=$((elapsed + 15))
done

if [ $elapsed -ge $timeout ]; then
    echo -e "${RED}Timeout waiting for eigenlayer setup to complete${NC}"
    echo "Checking container logs..."
    docker compose logs eigenlayer
    exit 1
fi

# Wait a bit more for file system writes to complete
sleep 10

# Verify deployment file was created
if [ ! -f .nodes/avs_deploy.json ]; then
    echo -e "${RED}Deployment file .nodes/avs_deploy.json was not created${NC}"
    echo "Checking eigenlayer logs:"
    docker compose logs eigenlayer
    echo "Checking ethereum logs:"  
    docker compose logs ethereum
    exit 1
fi

echo -e "${GREEN}Deployment file created successfully${NC}"
echo "Deployment file contents:"
cat .nodes/avs_deploy.json

cd "$PROJECT_ROOT"

# Step 4: Start contributors
echo -e "${YELLOW}Step 4: Starting contributors...${NC}"
cd commonware-avs-node

if ! source .env; then
    echo -e "${RED}Error: Failed to source commonware-avs-node/.env file${NC}"
    echo "Contents of commonware-avs-node/.env file:"
    cat .env
    exit 1
fi

# Start contributor 1
echo "Starting contributor 1..."
cargo run --release --quiet -- --key-file "$CONTRIBUTOR_1_KEYFILE" --port 3001 --orchestrator ../orchestrator.json > "$LOG_DIR/contributor1.log" 2>&1 &
PID1=$!
echo "contributor1:$PID1" >> "$PIDS_FILE"
echo "Contributor 1 started with PID: $PID1"

sleep 1

# Start contributor 2
echo "Starting contributor 2..."
cargo run --release --quiet -- --key-file "$CONTRIBUTOR_2_KEYFILE" --port 3002 --orchestrator ../orchestrator.json > "$LOG_DIR/contributor2.log" 2>&1 &
PID2=$!
echo "contributor2:$PID2" >> "$PIDS_FILE"
echo "Contributor 2 started with PID: $PID2"

sleep 1

# Start contributor 3
echo "Starting contributor 3..."
cargo run --release --quiet -- --key-file "$CONTRIBUTOR_3_KEYFILE" --port 3003 --orchestrator ../orchestrator.json > "$LOG_DIR/contributor3.log" 2>&1 &
PID3=$!
echo "contributor3:$PID3" >> "$PIDS_FILE"
echo "Contributor 3 started with PID: $PID3"

# Wait for contributors to initialize
echo "Waiting for contributors to initialize..."
sleep 15

cd "$PROJECT_ROOT"

# Step 5: Start orchestrator
echo -e "${YELLOW}Step 5: Starting orchestrator...${NC}"
if ! source .env; then
    echo -e "${RED}Error: Failed to source .env file${NC}"
    echo "Contents of .env file:"
    cat .env
    exit 1
fi

cargo run --release --quiet -- --key-file orchestrator.json --port 3000 > "$LOG_DIR/orchestrator.log" 2>&1 &
ORCHESTRATOR_PID=$!
echo "orchestrator:$ORCHESTRATOR_PID" >> "$PIDS_FILE"
echo "Orchestrator started with PID: $ORCHESTRATOR_PID"

# Wait for orchestrator to initialize
echo "Waiting for orchestrator to initialize..."
sleep 15

# Step 6: Wait and verify increments
echo -e "${YELLOW}Step 6: Waiting for signature aggregation and verifying increments...${NC}"

# Wait for at least 2 aggregation cycles (30 seconds each + buffer)
echo "Waiting for signature aggregation cycles to complete..."
echo "This will take approximately 2-3 minutes..."

# Build and run verification script
cd scripts
echo "Building verification script..."
cargo build --release --bin verify_increments --quiet
if ! source ../.env; then
    echo -e "${RED}Error: Failed to source ../.env file${NC}"
    echo "Contents of ../.env file:"
    cat ../.env
    exit 1
fi

# Set the deployment path relative to the scripts directory
export AVS_DEPLOYMENT_PATH="../eigenlayer-bls-local/.nodes/avs_deploy.json"
echo "AVS_DEPLOYMENT_PATH set to: $AVS_DEPLOYMENT_PATH"
echo "File exists check:"
ls -la "$AVS_DEPLOYMENT_PATH" || echo "File not found at $AVS_DEPLOYMENT_PATH"

cargo run --release --bin verify_increments

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Integration test PASSED! Counter was incremented successfully.${NC}"
    exit 0
else
    echo -e "${RED}❌ Integration test FAILED! Counter was not incremented as expected.${NC}"
    
    # Print recent logs for debugging
    echo -e "${YELLOW}Recent orchestrator logs:${NC}"
    tail -n 20 "$LOG_DIR/orchestrator.log" || echo "No orchestrator logs found"
    
    echo -e "${YELLOW}Recent contributor logs:${NC}"
    tail -n 10 "$LOG_DIR/contributor1.log" || echo "No contributor1 logs found"
    tail -n 10 "$LOG_DIR/contributor2.log" || echo "No contributor2 logs found"
    tail -n 10 "$LOG_DIR/contributor3.log" || echo "No contributor3 logs found"
    
    echo -e "${YELLOW}Recent eigenlayer setup logs:${NC}"
    docker compose logs --tail=50 eigenlayer || echo "No eigenlayer logs found"
    
    exit 1
fi 