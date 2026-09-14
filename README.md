# soroquest-contracts

Soroban smart contracts for the SoroQuest bounty platform.

## Overview

This repo contains the `soroquest-escrow` contract — the on-chain source of truth
for all bounties. It holds USDC in escrow and releases it when work is approved.

## Quick start

```bash
# Build
make build

# Test
make test

# Deploy to testnet
make deploy-testnet
```

## Contract addresses

- Testnet: see `deployments/testnet.json`
- Mainnet: see `deployments/mainnet.json`

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for full design documentation.

## License

MIT
