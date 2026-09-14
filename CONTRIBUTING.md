# Contributing

## Setup

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Add the WASM target: `rustup target add wasm32v1-none`
3. Install the Stellar CLI: https://developers.stellar.org/docs/tools/stellar-cli

## Development workflow

1. Make changes in `contracts/soroquest-escrow/src/`
2. Run `make test` to verify
3. Run `make fmt` before committing
4. Open a PR against `main`

## Testing

All tests live in `src/test.rs`. Run with `make test`.
No network required — tests use the Soroban in-process environment.
