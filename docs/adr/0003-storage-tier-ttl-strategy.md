# ADR 0003: Soroban State Storage Tier and TTL Strategy

## Status
Accepted

## Context
Soroban introduces a state archival mechanism where contract state is partitioned into three tiers:
- **Temporary Storage**: Ephemeral data evicted when TTL expires; cannot be restored.
- **Instance Storage**: Bound to the contract executable, shared across contract functions.
- **Persistent Storage**: Retained across ledgers, can be archived and restored, suitable for core user assets and balances.

## Decision
1. **Global Sequence ID**: Stored in `Instance` storage under `DataKey::BountyCount`. Contract instance storage is extended during routine contract calls.
2. **Bounty Records**: Each bounty is stored in `Persistent` storage under `DataKey::Bounty(u64)`.
   - Using persistent storage guarantees that active escrow records cannot be accidentally deleted by transient ledger churn.
   - Bounties can be restored if archived using Soroban restore footprint operations.
3. **Data Size Minimization**:
   - `Bounty` structs store only essential cryptographic identifiers, state enumerations, amounts, and string slices for titles/descriptions within reasonable byte limits, ensuring minimal ledger footprint and low transaction storage fees.

## Consequences
- Cost-effective storage scaling linearly with bounty volume.
- Safe archival resilience for high-value escrows.
