#!/bin/bash
if [ $# -ne 2 ]; then
    echo "Usage: $0 <password> <account>"
    exit 1
fi

PASSWORD="$1"
ACCOUNT="$2"
# Create a new tmux session
tmux new-session -d -s export_key

# Send the export command
tmux send-keys -t export_key "eigenlayer keys export --key-type bls $ACCOUNT" C-m

# Wait a bit and send "y"
sleep 1
tmux send-keys -t export_key "y" C-m

# Wait a bit and send password
sleep 1
tmux send-keys -t export_key "$PASSWORD" C-m

# Capture the output and format it
sleep 1
tmux capture-pane -t export_key -S - -E - -p | grep -A1 "Private key:" | tr -d 'Private key: \n'

# Kill the session
tmux kill-session -t export_key