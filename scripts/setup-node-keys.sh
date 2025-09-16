#!/bin/bash

# Wait for eigenlayer to create the operator keys
echo "Waiting for eigenlayer to generate operator keys..."
while [ ! -f ".nodes/operator_keys/testacc1.private.bls.key.json" ] || [ ! -f ".nodes/operator_keys/testacc2.private.bls.key.json" ] || [ ! -f ".nodes/operator_keys/testacc3.private.bls.key.json" ]; do
  sleep 5
  echo "Still waiting for operator keys..."
done

echo "Operator keys found. Creating node key files with proper format..."

# Extract private keys from the eigenlayer-generated files
KEY1=$(cat .nodes/operator_keys/testacc1.private.bls.key.json | jq -r '.privateKey')
KEY2=$(cat .nodes/operator_keys/testacc2.private.bls.key.json | jq -r '.privateKey')
KEY3=$(cat .nodes/operator_keys/testacc3.private.bls.key.json | jq -r '.privateKey')

# G2 public key coordinates (same for all nodes in test environment)
G2_X1="11573484064405516224823421838123830914352068825586086624198210333856287278863"
G2_X2="15813923616902403940433457154007641751038452797126613514626768548264821417527"
G2_Y1="8567886311428202827721503784509432473849014458061816488507357191782150509069"
G2_Y2="17949556893579689123064078966163614621858087168071980002870648396365788585440"

# Create node1.bls.key.json
cat > .nodes/operator_keys/node1.bls.key.json <<EOF
{
  "privateKey": "$KEY1",
  "g2_x1": "$G2_X1",
  "g2_x2": "$G2_X2",
  "g2_y1": "$G2_Y1",
  "g2_y2": "$G2_Y2",
  "port": "commonware-avs-node-1:3001"
}
EOF

# Create node2.bls.key.json
cat > .nodes/operator_keys/node2.bls.key.json <<EOF
{
  "privateKey": "$KEY2",
  "g2_x1": "$G2_X1",
  "g2_x2": "$G2_X2",
  "g2_y1": "$G2_Y1",
  "g2_y2": "$G2_Y2",
  "port": "3002"
}
EOF

# Create node3.bls.key.json
cat > .nodes/operator_keys/node3.bls.key.json <<EOF
{
  "privateKey": "$KEY3",
  "g2_x1": "$G2_X1",
  "g2_x2": "$G2_X2",
  "g2_y1": "$G2_Y1",
  "g2_y2": "$G2_Y2",
  "port": "3003"
}
EOF

echo "Node key files created successfully!"
ls -la .nodes/operator_keys/node*.bls.key.json