# SoroQuest Contracts

[![CI](https://github.com/soroquest-hq/soroquest-contracts/actions/workflows/ci.yml/badge.svg)](https://github.com/soroquest-hq/soroquest-contracts/actions/workflows/ci.yml)

The SoroQuest smart contract is an escrow system for managing bounties on the Stellar network. It holds USDC deposits securely until work is completed or the bounty is cancelled.

## Prerequisites

- Rust toolchain (`rustup`)
- `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- Stellar CLI (`cargo install --locked stellar-cli --features opt`)

## Building and Testing

To build the contract into a WebAssembly binary:

```bash
stellar contract build
```

To run the full test suite:

```bash
cargo test
```

## Documentation

- [Architecture](docs/architecture.md): High-level design and storage layout.
- [Contract Reference](docs/contract-reference.md): Public functions and errors.

## Deployed Addresses

| Network | Address | Explorer |
|---------|---------|----------|
| Testnet | `CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV` | [Stellar Expert](https://stellar.expert/explorer/testnet/contract/CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV) |
| Mainnet | (Pending deployment) | [Stellar Expert](https://stellar.expert/explorer/public) |
