# Changelog

All notable changes to the SoroQuest Escrow contract protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-28

### Added
- **Core Escrow Logic**:
  - `post_bounty`: Lock USDC tokens into contract with metadata, deadline, and unique sequential ID.
  - `claim_bounty`: Exclusive contributor reservation.
  - `approve_completion`: Instant on-chain release of escrowed funds to claimant upon owner approval.
  - `cancel_bounty`: Fund reclamation with strict deadline enforcement to protect active contributors.
- **Security & Invariants**:
  - Balance conservation invariant test suite verifying zero-loss fund accounting.
  - Pinned `ed25519-dalek` to `=2.2.0` in Cargo manifest to maintain non-std host target compatibility.
  - Non-owner authorization guards with explicit error codes (`Error(Contract, #5)`).
- **Deployment & Tooling**:
  - Deployed verified contract to Stellar Testnet at `CA7C6LGWPTZVUTZX7EXJVLWCQG6DVWPROBPO3FGXYKEY4W5H4HRPO5UV`.
  - Automated deployment and verification bash scripts in `scripts/`.
  - Comprehensive Architecture Decision Records (ADRs 0001, 0002, 0003).
  - Gas and CPU instruction benchmark documentation.
