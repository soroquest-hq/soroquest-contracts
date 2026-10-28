#!/usr/bin/env bash
set -euo pipefail

# SoroQuest Contract Verification Script
CONTRACT_ID="${1:-CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV}"
NETWORK="${2:-testnet}"

echo "=== Verifying SoroQuest Escrow Contract on Stellar ==="
echo "Contract ID: $CONTRACT_ID"
echo "Network:     $NETWORK"

EXPLORER_URL="https://stellar.expert/explorer/${NETWORK}/contract/${CONTRACT_ID}"
echo "Stellar Expert URL: $EXPLORER_URL"

# Test RPC connectivity
RPC_URL="https://soroban-testnet.stellar.org"
HEALTH_CHECK=$(curl -s -X POST "$RPC_URL" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' || echo "")

if echo "$HEALTH_CHECK" | grep -q "healthy"; then
    echo "Soroban RPC Status: Healthy"
else
    echo "Warning: RPC returned unexpected response: $HEALTH_CHECK"
fi

echo "Contract verification check completed."
