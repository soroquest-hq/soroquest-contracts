# ADR 0002: Token Interface and Stellar Asset Contract (SAC) Compatibility

## Status
Accepted

## Context
SoroQuest bounties are denominated in USDC or other Stellar assets. Soroban handles both native Soroban tokens (custom wasm tokens implementing the SEP-41 token standard) and classic Stellar assets bridged via the Stellar Asset Contract (SAC).

The escrow contract must reliably transfer tokens from the bounty creator to the escrow contract upon posting, and from the escrow contract to the claimant or creator upon approval or cancellation.

## Decision
1. We interact with tokens exclusively through the official `soroban_sdk::token::Client`.
2. All token transfers utilize standard authorization semantics:
   - On `post_bounty`: `client.transfer(&owner, &contract_address, &amount)` where `owner.require_auth()` is enforced by the caller signing the Soroban transaction.
   - On `approve_bounty`: `client.transfer(&contract_address, &claimant, &amount)`. Since the contract is transferring its own funds, the Soroban host grants authorization automatically without external signatures.
   - On `cancel_bounty`: `client.transfer(&contract_address, &owner, &amount)`.
3. Token addresses are validated at the contract boundary to ensure non-zero amounts (`amount > 0`).

## Consequences
- 100% interoperability with native Stellar USDC SAC instances (`CBIELTK...` on Testnet, `CCW67...` on Mainnet).
- Clean separation between business logic and underlying asset ledger mechanics.
