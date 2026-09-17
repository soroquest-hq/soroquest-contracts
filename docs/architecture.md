# Architecture

## Overall Design

The SoroQuest Escrow contract acts as a trustless intermediary between bounty owners (who fund tasks) and claimants (who complete tasks). 

The lifecycle of a bounty is:
1. **Post**: Owner calls `post_bounty`. USDC is transferred from the owner to the contract's own address. Status becomes `Open`.
2. **Claim**: A user calls `claim_bounty` to indicate they are working on it. Status becomes `Claimed`.
3. **Approve**: The owner reviews the work off-chain and calls `approve_completion`. USDC is transferred from the contract to the claimant. Status becomes `Completed`.

Alternatively, a bounty can be **Cancelled**:
- If `Open`, the owner can cancel at any time.
- If `Claimed`, the owner can only cancel if the `claim_deadline` has passed.
In both cases, USDC is returned to the owner.

## Storage Layout

The contract uses `Persistent` storage for all data. Every time a bounty is created or modified, its TTL (Time To Live) is extended by `535_000` ledgers (roughly 1 year on Stellar mainnet).

We use two `StorageKey` variants:
- `StorageKey::BountyCount`: Stores a single `u64` tracking the total number of bounties ever created. This serves as an auto-incrementing ID for new bounties.
- `StorageKey::Bounty(u64)`: Stores a `Bounty` struct for the given ID.

## Token Transfer Pattern

The contract relies on the standard Stellar Asset Contract (SAC) interface. 
When interacting with tokens (like USDC), it uses `soroban_sdk::token::Client`.
All transfers involve the contract's own address (`env.current_contract_address()`) as either the destination (when escrowing) or the source (when releasing funds).

## Authorization Flow

All state-changing functions enforce authorization using the `require_auth()` method on the `Address` type. This ensures that the caller has cryptographically signed the transaction approving the specific contract invocation.

Pattern:
```rust
pub fn some_action(env: Env, user: Address, bounty_id: u64) {
    user.require_auth(); // Halts execution if 'user' did not authorize this call
    
    // ... custom logic, e.g., checking if 'user' is the bounty owner ...
}
```

## Event Schema

The contract emits events for all state changes to allow off-chain indexers to track the platform's state. All events use a two-element topic tuple: `(symbol_short!("action"), bounty_id)`.

- `posted`: Emitted on `post_bounty`. Payload: `(owner, amount, token, claim_deadline)`.
- `claimed`: Emitted on `claim_bounty`. Payload: `claimant`.
- `completed`: Emitted on `approve_completion`. Payload: `(claimant, amount)`.
- `cancelled`: Emitted on `cancel_bounty`. Payload: `(owner, amount)`.

## Adding a New Function

1. Define the function signature in `src/lib.rs`.
2. If it modifies state, ensure it takes an `Address` parameter for the actor and calls `require_auth()`.
3. Use the storage helpers in `src/storage.rs` to read/write data.
4. If it emits an event, add a new helper in `src/events.rs`.
5. Add comprehensive unit tests in `src/test.rs`.
