# Docker Compose Setup for Commonware AVS

This setup uses pre-built Docker images from GitHub Container Registry to run the commonware AVS infrastructure.

## Prerequisites

- Docker and Docker Compose installed
- `jq` command-line tool installed (for key file processing)

## Images Used

- `ghcr.io/breadchaincoop/commonware-avs-router:dev` - AVS Router
- `ghcr.io/breadchaincoop/commonware-avs-node:v0.1.0` - AVS Nodes (3 instances)
- `ghcr.io/breadchaincoop/ethereum:dev` - Ethereum node (Anvil)
- `ghcr.io/breadchaincoop/eigenlayer:dev` - EigenLayer setup
- `ghcr.io/layr-labs/cerberus:0.0.2` - Signer service

## Setup Instructions

1. **Copy environment file:**
   ```bash
   cp example.env .env
   ```

2. **Configure environment for local mode:**
   ```bash
   # Set up for local testing
   sed -i '' 's|^HTTP_RPC=.*|HTTP_RPC=http://localhost:8545|' .env
   sed -i '' 's|^WS_RPC=.*|WS_RPC=ws://localhost:8545|' .env
   sed -i '' 's|^ENVIRONMENT=.*|ENVIRONMENT=LOCAL|' .env
   ```

3. **Create config file:**
   ```bash
   cp config/config.example.json config/config.json
   ```

4. **Start the infrastructure:**
   ```bash
   docker compose up -d
   ```

5. **Wait for EigenLayer setup to complete (about 60 seconds):**
   ```bash
   # Check eigenlayer logs
   docker compose logs eigenlayer --tail=20
   
   # Verify deployment file was created
   ls -la .nodes/avs_deploy.json
   ```

6. **Set up node key files:**
   ```bash
   ./scripts/setup-node-keys.sh
   ```

7. **Restart AVS nodes with proper keys:**
   ```bash
   docker compose restart commonware-avs-node-1 commonware-avs-node-2 commonware-avs-node-3
   ```

## Port Mappings

- Ethereum RPC: `8545`
- AVS Router: `4000` (internal: 3000)
- AVS Node 1: `4001` (internal: 3001)
- AVS Node 2: `4002` (internal: 3002)
- AVS Node 3: `4003` (internal: 3003)
- Cerberus Metrics: `9081`
- Cerberus gRPC: `50051`

## Checking Status

```bash
# View all services
docker compose ps

# Check router logs
docker compose logs commonware-avs-router --tail=20

# Check node logs
docker compose logs commonware-avs-node-1 --tail=20

# Follow logs in real-time
docker compose logs -f
```

## Troubleshooting

### Nodes failing with "key file not found"
Run the setup script again:
```bash
./scripts/setup-node-keys.sh
docker compose restart commonware-avs-node-1 commonware-avs-node-2 commonware-avs-node-3
```

### Connection failures between nodes
This is normal initially as nodes discover each other. The P2P network will establish connections after a few retry attempts.

### Ports already in use
The services use ports 4000-4003 to avoid conflicts. If these are in use, modify the port mappings in docker-compose.yml.

## Stopping Services

```bash
docker compose down
```

To also remove volumes:
```bash
docker compose down -v
```

## Notes

- The setup uses test keys and is intended for development/testing only
- The eigenlayer container automatically generates operator keys and deploys contracts
- Node keys require a specific format with G2 public key coordinates
- The orchestrator_with_g2.json file contains the orchestrator key configuration