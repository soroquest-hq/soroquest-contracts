# ARCHITECTURE ESSENTIALS — soroquest-contracts

## Stack
- Rust + `soroban-sdk` latest stable
- Build: `stellar contract build` (targets `wasm32v1-none`)
- Tests: Soroban in-process test env (no network needed)
- Deploy: `stellar` CLI

## Key types
```rust
Bounty { id, owner, title, description, amount, token, status, claimant, created_at, claim_deadline }
BountyStatus { Open | Claimed | Completed | Cancelled }
StorageKey { BountyCount, Bounty(u64) }
SoroQuestError { BountyNotFound=1, BountyNotOpen=2, BountyNotClaimed=3, AlreadyClaimed=4,
                 NotOwner=5, DeadlineNotPassed=6, InvalidAmount=7, InvalidDeadline=8, TransferFailed=9 }
```

## Storage
- All bounties: `env.storage().persistent()` — must extend TTL on every write
- Counter key: `StorageKey::BountyCount` (u64)
- Bounty key: `StorageKey::Bounty(id)`

## Public functions
```
post_bounty(owner, title, desc, amount, token, deadline) -> u64
claim_bounty(claimant, bounty_id)
approve_completion(owner, bounty_id)
cancel_bounty(owner, bounty_id)
get_bounty(bounty_id) -> Bounty
list_bounties(status?, offset, limit) -> Vec<Bounty>
get_bounty_count() -> u64
```

## Auth rules — enforce all
- `post_bounty` → `owner.require_auth()` + transfer USDC owner→contract
- `claim_bounty` → `claimant.require_auth()` + bounty must be Open
- `approve_completion` → `owner.require_auth()` + transfer USDC contract→claimant
- `cancel_bounty` → `owner.require_auth()` + Open OR (Claimed AND deadline passed)

## Token transfers — use SAC interface only
```rust
TokenClient::new(&env, &token).transfer(&from, &to, &amount);
```

## Events — emit on every state change
```
("posted",    bounty_id) → (owner, amount, token, deadline)
("claimed",   bounty_id) → claimant
("completed", bounty_id) → (claimant, amount)
("cancelled", bounty_id) → (owner, amount)
```

## USDC addresses
```
Testnet: CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA
Mainnet: CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7EJJUD
```

## Critical rules
- Never `unwrap()` or `panic!` in production paths — return `SoroQuestError`
- Never hardcode token address in logic — store it on the Bounty struct
- Never partial pay — always full `amount` or nothing
- Extend persistent storage TTL on every write
- Emit event after every state change, before returning

## Deployment outputs
```
deployments/testnet.json → { "contract_id": "C..." }
deployments/mainnet.json → { "contract_id": "C..." }
```
