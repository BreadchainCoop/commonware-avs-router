#!/bin/bash

# Docker Compose Validation Script
# This script validates the docker-compose.yml configuration

set -e

echo "🔍 Validating Docker Compose configuration..."

# Check if docker-compose.yml exists
if [ ! -f "docker-compose.yml" ]; then
    echo "❌ docker-compose.yml not found"
    exit 1
fi

# Validate docker-compose.yml syntax
echo "✅ Checking docker-compose.yml syntax..."
if ! docker compose config > /dev/null 2>&1; then
    echo "❌ docker-compose.yml has syntax errors"
    docker compose config
    exit 1
fi

# Check if .env.example exists
if [ ! -f ".env.example" ]; then
    echo "❌ .env.example not found"
    exit 1
fi

# Check if config directories exist
echo "✅ Checking config directory structure..."
required_dirs=(
    "config/router"
    "config/node" 
    "config/eigenlayer"
    "config/avs-keys"
    "config/avs-keys/operator_keys"
)

for dir in "${required_dirs[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "❌ Required directory missing: $dir"
        exit 1
    fi
done

# Check if required config files exist
echo "✅ Checking required config files..."
required_files=(
    "config/eigenlayer/.env"
    "config/eigenlayer/config.json"
    "config/router/orchestrator.json"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Required config file missing: $file"
        exit 1
    fi
done

# Validate docker-compose services
echo "✅ Checking service configuration..."
services=(
    "ethereum"
    "eigenlayer"
    "signer"
    "avs-router"
    "avs-node-contributor-1"
    "avs-node-contributor-2"
    "avs-node-contributor-3"
)

config_output=$(docker compose config)
for service in "${services[@]}"; do
    if ! echo "$config_output" | grep -q "  $service:"; then
        echo "❌ Service missing from configuration: $service"
        exit 1
    fi
done

# Check port configurations
echo "✅ Checking port configurations..."
expected_ports=(
    "8545:8545"    # ethereum
    "3000:3000"    # avs-router
    "3001:3001"    # contributor-1
    "3002:3002"    # contributor-2 
    "3003:3003"    # contributor-3
)

for port in "${expected_ports[@]}"; do
    if ! echo "$config_output" | grep -q "$port"; then
        echo "⚠️  Expected port mapping not found: $port"
    fi
done

# Check if network is defined
if ! echo "$config_output" | grep -q "avsnet:"; then
    echo "❌ avsnet network not defined"
    exit 1
fi

# Validate environment variable usage
echo "✅ Checking environment variable usage..."
required_env_vars=(
    "PRIVATE_KEY"
    "ENVIRONMENT"
    "CERBERUS_GRPC_PORT"
    "CERBERUS_METRICS_PORT"
)

for var in "${required_env_vars[@]}"; do
    if ! echo "$config_output" | grep -q "\${$var"; then
        echo "⚠️  Environment variable not used in compose: $var"
    fi
done

echo ""
echo "🎉 Docker Compose configuration validation completed successfully!"
echo ""
echo "📋 Summary:"
echo "   - docker-compose.yml syntax is valid"
echo "   - All required services are defined"
echo "   - Config directory structure is correct"
echo "   - Required config files exist"
echo "   - Network configuration is present"
echo ""
echo "🚀 To start the environment:"
echo "   1. Copy .env.example to .env"
echo "   2. Edit .env and set your PRIVATE_KEY"
echo "   3. Run: docker compose up -d"
echo ""
echo "📖 For detailed usage instructions, see the README.md file"