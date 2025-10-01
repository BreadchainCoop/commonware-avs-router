# Commonware AVS Router

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ghcr.io/breadchaincoop/commonware--avs--router-blue.svg)](https://github.com/BreadchainCoop/commonware-avs-router/pkgs/container/commonware-avs-router)

A BLS signature aggregation protocol with onchain execution for EigenLayer AVS operators.

## Overview

The router coordinates multiple operators to sign messages, aggregates their signatures when a threshold is reached, and executes the result onchain.

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Git

### Local Development

1. **Configure environment:**
```bash
cp example.env .env
```

For LOCAL mode (default), the example.env is pre-configured. You'll need to set a private key:
```bash
# Use Anvil's default test key for local development
echo "PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" >> .env
echo "FUNDED_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" >> .env
```

2. **Start all services:**
```bash
docker compose up -d
```

This will automatically pull the latest pre-built images from the GitHub Container Registry (ghcr.io) and start:
- Ethereum node (Anvil fork)
- EigenLayer contract deployment
- 3 operator nodes
- Router/orchestrator
- Signer service

3. **Monitor services:**
```bash
# View logs
docker compose logs -f router

# Check service status
docker compose ps
```

### Stop Services

```bash
# Stop all services
docker compose down

# Stop and remove volumes (clean state)
docker compose down -v
```

### Building from Source (Development Only)

If you're developing the router and want to test local changes:

```bash
# Build the router image locally
docker build -t ghcr.io/breadchaincoop/commonware-avs-router:dev .

# Run with locally built image
docker compose up -d
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
- `INGRESS_TIMEOUT_MS`: Timeout for waiting for ingress tasks in milliseconds (default: 30000)

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

1. **Enable ingress in .env:**
```bash
INGRESS=true
```

2. **Restart the router:**
```bash
docker compose restart router
```

3. **Trigger tasks via HTTP:**
```bash
curl -X POST http://localhost:8080/trigger \
  -H "Content-Type: application/json" \
  -d '{"body": {"metadata": {"request_id": "1", "action": "increment"}}}'
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

## Kubernetes Deployment

### Helm Charts

This project includes Helm charts for easy deployment to Kubernetes clusters. The charts support both development and production environments with comprehensive configuration options.

#### Prerequisites

- Kubernetes cluster (version 1.19+)
- Helm 3.0+
- kubectl configured to access your cluster

#### Quick Start

1. **Install the chart with default values:**
```bash
helm install my-router charts/commonware-avs-router
```

2. **Install with custom values:**
```bash
helm install my-router charts/commonware-avs-router -f custom-values.yaml
```

3. **Install for development:**
```bash
helm install dev-router charts/commonware-avs-router \
  -f charts/commonware-avs-router/examples/development-values.yaml
```

4. **Install for production:**
```bash
helm install prod-router charts/commonware-avs-router \
  -f charts/commonware-avs-router/examples/production-values.yaml
```

#### Configuration

The Helm chart supports extensive configuration through `values.yaml`. Key configuration categories include:

##### Container Configuration
```yaml
image:
  repository: ghcr.io/breadchaincoop/commonware-avs-router
  tag: "latest"
  pullPolicy: IfNotPresent

replicaCount: 1
```

##### Service and Networking
```yaml
service:
  type: ClusterIP
  port: 3000

ingress:
  enabled: false
  className: ""
  annotations: {}
  hosts:
    - host: chart-example.local
      paths: ["/"]
  tls: []
```

##### Environment Variables
```yaml
env:
  HTTP_RPC: "https://ethereum-holesky.publicnode.com"
  WS_RPC: "wss://ethereum-holesky.publicnode.com"
  ENVIRONMENT: "TESTNET"
  AGGREGATION_FREQUENCY: "30"
  THRESHOLD: "3"
  INGRESS: "false"
```

##### Secrets Management
```yaml
# Environment variables from secrets
envFromSecrets:
  PRIVATE_KEY:
    secretName: "router-secrets"
    key: "private-key"
  FUNDED_KEY:
    secretName: "router-secrets"
    key: "funded-key"

# Create secrets directly in chart
secrets:
  enabled: true
  data:
    private-key: "<base64-encoded-private-key>"
    funded-key: "<base64-encoded-funded-key>"
```

##### Resource Management
```yaml
resources:
  limits:
    cpu: 500m
    memory: 512Mi
  requests:
    cpu: 250m
    memory: 256Mi
```

##### Persistent Storage
```yaml
persistence:
  config:
    enabled: true
    storageClass: "fast-ssd"
    accessMode: ReadWriteOnce
    size: 10Gi
    mountPath: /app/config
    
  keys:
    enabled: true
    storageClass: "fast-ssd"  
    accessMode: ReadWriteOnce
    size: 5Gi
    mountPath: /app/keys
```

##### Health Checks
```yaml
livenessProbe:
  enabled: true
  tcpSocket:
    port: 3000
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  enabled: true
  tcpSocket:
    port: 3000
  initialDelaySeconds: 5
  periodSeconds: 5
```

##### Autoscaling
```yaml
autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 10
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80
```

#### Deployment Examples

##### Development Environment
```bash
# Install with development settings
helm install dev-router charts/commonware-avs-router \
  --set image.tag=latest \
  --set image.pullPolicy=Always \
  --set replicaCount=1 \
  --set service.type=NodePort \
  --set env.ENVIRONMENT=LOCAL \
  --set env.AGGREGATION_FREQUENCY=10
```

##### Production Environment
```bash
# Create production secrets first
kubectl create secret generic router-secrets \
  --from-literal=private-key="your-private-key" \
  --from-literal=funded-key="your-funded-key"

# Install with production settings
helm install prod-router charts/commonware-avs-router \
  --set image.tag=v1.0.0 \
  --set replicaCount=3 \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=avs-router.example.com \
  --set resources.requests.cpu=500m \
  --set resources.requests.memory=512Mi \
  --set envFromSecrets.PRIVATE_KEY.secretName=router-secrets \
  --set envFromSecrets.PRIVATE_KEY.key=private-key
```

##### Testnet Deployment
```bash
helm install testnet-router charts/commonware-avs-router \
  --set env.ENVIRONMENT=TESTNET \
  --set env.HTTP_RPC=https://ethereum-holesky.publicnode.com \
  --set env.WS_RPC=wss://ethereum-holesky.publicnode.com \
  --set persistence.config.enabled=true \
  --set persistence.keys.enabled=true
```

#### Management Commands

##### Upgrade deployment:
```bash
helm upgrade my-router charts/commonware-avs-router -f updated-values.yaml
```

##### Check deployment status:
```bash
helm status my-router
kubectl get pods -l app.kubernetes.io/name=commonware-avs-router
```

##### View logs:
```bash
kubectl logs -l app.kubernetes.io/name=commonware-avs-router -f
```

##### Uninstall:
```bash
helm uninstall my-router
```

#### Security Considerations

The Helm chart follows security best practices:

- **Non-root execution**: Containers run as user ID 1000
- **Read-only root filesystem**: Container filesystem is read-only where possible
- **Dropped capabilities**: All Linux capabilities are dropped
- **Service account**: Dedicated service account with minimal permissions
- **Secret management**: Sensitive data stored in Kubernetes secrets
- **Network policies**: Optional network policies for traffic control

#### Monitoring and Observability

The chart supports integration with monitoring systems:

```yaml
# Enable service monitor for Prometheus
serviceMonitor:
  enabled: true
  labels:
    prometheus: kube-prometheus
  interval: 30s
  path: /metrics

# Pod annotations for metric scraping
podAnnotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "3000"
  prometheus.io/path: "/metrics"
```

#### Troubleshooting

1. **Check pod status:**
```bash
kubectl describe pod -l app.kubernetes.io/name=commonware-avs-router
```

2. **View container logs:**
```bash
kubectl logs -l app.kubernetes.io/name=commonware-avs-router --previous
```

3. **Debug configuration:**
```bash
helm template my-router charts/commonware-avs-router -f values.yaml
```

4. **Validate chart:**
```bash
helm lint charts/commonware-avs-router
```

For complete configuration options, see the [values.yaml](charts/commonware-avs-router/values.yaml) file.
