#!/bin/bash

# Configuration
ENDPOINT_URL="http://localhost:8080/trigger"  # Adjust port as needed
INTERVAL=30  # seconds

# Sample data - adjust these values as needed
REQUESTER_ADDRESS="0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6"
SIGNATURE="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1b"
BODY_ADDRESS="0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6"

# Counter for tracking requests
counter=1

echo "Starting to send requests to $ENDPOINT_URL every $INTERVAL seconds..."
echo "Press Ctrl+C to stop"
echo ""

while true; do
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] Sending request #$counter"
    
    # Create JSON payload
    json_data=$(cat <<EOF
{
    "requester": "$REQUESTER_ADDRESS",
    "signature": "$SIGNATURE",
    "body": {
        "address": "$BODY_ADDRESS",
        "number": $counter
    }
}
EOF
)

    # Send POST request
    response=$(curl -s -w "\nHTTP_STATUS:%{http_code}" \
        -X POST \
        -H "Content-Type: application/json" \
        -d "$json_data" \
        "$ENDPOINT_URL")

    # Extract HTTP status code
    http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d':' -f2)
    response_body=$(echo "$response" | sed '/HTTP_STATUS:/d')

    # Display results
    if [ "$http_status" = "200" ]; then
        echo "✅ Success (HTTP $http_status): $response_body"
    else
        echo "❌ Error (HTTP $http_status): $response_body"
    fi
    
    echo ""
    
    # Increment counter
    ((counter++))
    
    # Wait for next interval
    sleep $INTERVAL
done 