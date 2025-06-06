#!/bin/bash

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Error: jq is not installed. Please install it first."
    exit 1
fi

# Path to the avs_deploy.json file
DEPLOY_FILE="eigenlayer-bls-local/.nodes/avs_deploy.json"
ENV_FILE="commonware-avs-node/.env"

# Check if the deploy file exists
if [ ! -f "$DEPLOY_FILE" ]; then
    echo "Error: $DEPLOY_FILE not found"
    exit 1
fi

# Check if the .env file exists
if [ ! -f "$ENV_FILE" ]; then
    echo "Error: $ENV_FILE not found"
    exit 1
fi

# Create a temporary file
TMP_FILE=$(mktemp)

# Read the current .env file and update the variables
while IFS= read -r line || [ -n "$line" ]; do
    if [[ $line == REGISTRY_COORDINATOR_ADDRESS=* ]]; then
        echo "REGISTRY_COORDINATOR_ADDRESS=$(jq -r '.addresses.registryCoordinator' "$DEPLOY_FILE")"
    elif [[ $line == BLS_APK_REGISTRY_ADDRESS=* ]]; then
        echo "BLS_APK_REGISTRY_ADDRESS=$(jq -r '.addresses.blsapkRegistry' "$DEPLOY_FILE")"
    elif [[ $line == COUNTER_ADDRESS=* ]]; then
        echo "COUNTER_ADDRESS=$(jq -r '.addresses.counter' "$DEPLOY_FILE")"
    else
        echo "$line"
    fi
done < "$ENV_FILE" > "$TMP_FILE"

# Replace the original .env file with the updated one
mv "$TMP_FILE" "$ENV_FILE"

echo "Environment variables have been updated successfully!" 