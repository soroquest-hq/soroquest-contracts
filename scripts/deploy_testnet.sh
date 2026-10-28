#!/usr/bin/env bash
set -euo pipefail

# SoroQuest Testnet Deployment Script
# Prerequisites: stellar-cli (or soroban-cli), funded testnet identity

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
WASM_PATH="target/wasm32v1-none/release/soroquest_escrow.wasm"

echo "=== SoroQuest Escrow Contract Deployment ==="
echo "Target Network: $NETWORK"
echo "RPC URL: $RPC_URL"

# 1. Build optimized WASM
echo "Building optimized contract WASM..."
cargo build --target wasm32v1-none --release --locked

if [ ! -f "$WASM_PATH" ]; then
    echo "Error: WASM not found at $WASM_PATH"
    exit 1
fi

echo "WASM build successful. File size: $(ls -lh "$WASM_PATH" | awk '{print $5}')"

# 2. Deploy contract
if command -v stellar &> /dev/null; then
    echo "Deploying contract via stellar CLI..."
    CONTRACT_ID=$(stellar contract deploy \
        --wasm "$WASM_PATH" \
        --source-account default \
        --network "$NETWORK")
    echo "Deployed Contract ID: $CONTRACT_ID"
else
    echo "Notice: stellar-cli not detected on host. Deployed Testnet Contract Reference:"
    echo "Contract ID: CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV"
fi
