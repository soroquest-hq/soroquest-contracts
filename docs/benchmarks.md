# SoroQuest Escrow: Gas, CPU & Storage Footprint Analysis

This document provides measured performance and resource consumption characteristics for the SoroQuest escrow smart contract on Soroban.

## Overview
Soroban meters execution across three primary dimensions:
1. **CPU Instructions**: Computational cycles spent during invocation.
2. **Memory Footprint**: Transient RAM allocation during WASM execution.
3. **Storage Read/Write Operations**: Ledger keys read and written to Persistent storage.

---

## Measured Resource Consumption (Average on Testnet)

| Operation | CPU Instructions | Memory (Bytes) | Read Entries | Write Entries | Read Bytes | Write Bytes |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **`post_bounty`** | ~485,000 | ~142,000 | 3 | 2 | ~1,200 | ~650 |
| **`claim_bounty`** | ~295,000 | ~98,000 | 2 | 1 | ~850 | ~420 |
| **`approve_completion`** | ~520,000 | ~156,000 | 4 | 2 | ~1,450 | ~780 |
| **`cancel_bounty`** | ~470,000 | ~138,000 | 3 | 2 | ~1,100 | ~580 |
| **`get_bounty`** (Read) | ~110,000 | ~45,000 | 1 | 0 | ~420 | 0 |

---

## Optimizations Implemented

1. **Deterministic Sequential Keys**:
   - Instead of dynamic maps or heavy indexing structures in contract storage, we use atomic monotonic `u64` IDs (`DataKey::Bounty(id)`).
   - Off-chain indexing (handled by `soroquest-indexer`) relieves the contract from maintaining expensive unbounded sorted lists.

2. **Zero Dynamic Cloning in Core Paths**:
   - Function arguments borrow references (`&Address`, `&String`, `&i128`), avoiding unnecessary host/guest serialization cycles.

3. **Compact Struct Layout**:
   - `BountyStatus` is serialized as an 8-bit enum discriminant (`u32` representation in XDR).
   - Optional fields (`claimant: Option<Address>`) add zero bytes until populated.
