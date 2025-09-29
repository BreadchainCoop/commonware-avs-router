# Commonware AVS Router Helm Chart

This Helm chart deploys the Commonware AVS Router to Kubernetes clusters.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+

## Installation

### Quick Start

```bash
# Install with default configuration
helm install my-router .

# Install with custom values
helm install my-router . -f my-values.yaml
```

### Example Configurations

The `examples/` directory contains pre-configured values files:

- `development-values.yaml` - Local development setup
- `production-values.yaml` - Production deployment with high availability

```bash
# Development deployment
helm install dev-router . -f examples/development-values.yaml

# Production deployment  
helm install prod-router . -f examples/production-values.yaml
```

## Configuration

See [values.yaml](values.yaml) for all available configuration options.

### Required Configuration

The application requires a private key for blockchain operations:

```yaml
envFromSecrets:
  PRIVATE_KEY:
    secretName: "router-secrets"
    key: "private-key"
```

Create the secret before deployment:

```bash
kubectl create secret generic router-secrets \
  --from-literal=private-key="your-private-key-here"
```

### Environment-Specific Settings

#### Local Development
```yaml
env:
  ENVIRONMENT: "LOCAL"
  HTTP_RPC: "http://localhost:8545"
  WS_RPC: "ws://localhost:8545"
```

#### Testnet (Holesky)
```yaml
env:
  ENVIRONMENT: "TESTNET"
  HTTP_RPC: "https://ethereum-holesky.publicnode.com"
  WS_RPC: "wss://ethereum-holesky.publicnode.com"
```

## Upgrading

```bash
helm upgrade my-router . -f values.yaml
```

## Uninstalling

```bash
helm uninstall my-router
```

## Troubleshooting

1. **Validate configuration:**
   ```bash
   helm lint .
   ```

2. **Test template rendering:**
   ```bash
   helm template test . -f values.yaml
   ```

3. **Check pod status:**
   ```bash
   kubectl get pods -l app.kubernetes.io/name=commonware-avs-router
   ```

For more information, see the main [README](../README.md#kubernetes-deployment).