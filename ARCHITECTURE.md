# ARCHITECTURE — soroquest-contracts

## Tech stack

| Layer | Choice | Reason |
|---|---|---|
| Language | Rust | Only supported language for Soroban contracts |
| SDK | `soroban-sdk` latest stable | Official SDK, required for host function access |
| Token standard | Stellar Asset Contract (SAC) | USDC on Stellar is a SAC; use the token interface |
| Build tool | `stellar contract build` | Targets `wasm32v1-none`, required by Soroban runtime |
| Test framework | `soroban-sdk` test environment | In-process VM, no network needed for tests |
| Deploy tool | `stellar` CLI | Official deployment and invocation tool |

## Contract structure

```
contracts/
└── soroquest-escrow/
    └── src/
        ├── lib.rs          # Public contract interface, function entry points
        ├── storage.rs      # Storage keys, data types, read/write helpers
        ├── events.rs       # Event emission helpers
        ├── error.rs        # SoroQuestError enum, all named variants
        └── test.rs         # Full test suite
```

## Data model

### `Bounty` struct

```rust
#[contracttype]
pub struct Bounty {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub description: String,
    pub amount: i128,
    pub token: Address,
    pub status: BountyStatus,
    pub claimant: Option<Address>,
    pub created_at: u64,        // ledger sequence at post time
    pub claim_deadline: u64,    // ledger sequence; 0 = no deadline
}
```

### `BountyStatus` enum

```rust
#[contracttype]
pub enum BountyStatus {
    Open,
    Claimed,
    Completed,
    Cancelled,
}
```

### `SoroQuestError` enum

```rust
#[contracterror]
pub enum SoroQuestError {
    BountyNotFound = 1,
    BountyNotOpen = 2,
    BountyNotClaimed = 3,
    AlreadyClaimed = 4,
    NotOwner = 5,
    DeadlineNotPassed = 6,
    InvalidAmount = 7,
    InvalidDeadline = 8,
    TransferFailed = 9,
}
```

## Storage layout

Soroban uses key-value storage. All keys are `#[contracttype]` enums.

```rust
#[contracttype]
pub enum StorageKey {
    BountyCount,            // u64 — total bounties ever posted
    Bounty(u64),            // Bounty struct, keyed by id
}
```

`BountyCount` is the auto-increment counter and the next bounty's ID.

All bounty storage uses `env.storage().persistent()` — bounties must
survive TTL expiration. The contract must extend TTL on every write.

## Public interface

```rust
pub trait SoroQuestTrait {
    fn post_bounty(
        env: Env,
        owner: Address,
        title: String,
        description: String,
        amount: i128,
        token: Address,
        claim_deadline: u64,
    ) -> Result<u64, SoroQuestError>;

    fn claim_bounty(
        env: Env,
        claimant: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError>;

    fn approve_completion(
        env: Env,
        owner: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError>;

    fn cancel_bounty(
        env: Env,
        owner: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError>;

    fn get_bounty(env: Env, bounty_id: u64) -> Result<Bounty, SoroQuestError>;

    fn list_bounties(
        env: Env,
        status: Option<BountyStatus>,
        offset: u64,
        limit: u64,
    ) -> Vec<Bounty>;

    fn get_bounty_count(env: Env) -> u64;
}
```

## Authorization flow

Every mutating function follows this pattern:

```rust
// 1. Require auth from the relevant address
owner.require_auth();

// 2. Validate business rules
let bounty = get_bounty(&env, bounty_id)?;
if bounty.owner != owner { return Err(SoroQuestError::NotOwner); }
if bounty.status != BountyStatus::Open { return Err(SoroQuestError::BountyNotOpen); }

// 3. Execute state change
// 4. Transfer tokens via SAC interface
// 5. Emit event
```

## Token transfer pattern

SoroQuest uses the standard Soroban token interface for all USDC
transfers. Never implement custom token logic.

```rust
use soroban_sdk::token::Client as TokenClient;

// On post_bounty — pull funds from owner into contract
let token = TokenClient::new(&env, &token_address);
token.transfer(&owner, &env.current_contract_address(), &amount);

// On approve_completion — push funds from contract to claimant
token.transfer(&env.current_contract_address(), &claimant, &amount);

// On cancel_bounty — push funds from contract back to owner
token.transfer(&env.current_contract_address(), &owner, &amount);
```

## Event schema

```rust
// bounty_posted
env.events().publish(
    (symbol_short!("posted"), bounty_id),
    (owner, amount, token, claim_deadline)
);

// bounty_claimed
env.events().publish(
    (symbol_short!("claimed"), bounty_id),
    claimant
);

// bounty_completed
env.events().publish(
    (symbol_short!("completed"), bounty_id),
    (claimant, amount)
);

// bounty_cancelled
env.events().publish(
    (symbol_short!("cancelled"), bounty_id),
    (owner, amount)
);
```

## USDC contract addresses

```
Testnet:  CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA
Mainnet:  CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7EJJUD
```

Always parameterise the token address — never hardcode it in contract
logic. Pass it as a parameter to `post_bounty` and store it on the
`Bounty` struct. This keeps the contract token-agnostic for future use.

## Deployment

```
deployments/
├── testnet.json    { "contract_id": "C...", "deployed_at": "<ledger>" }
└── mainnet.json    { "contract_id": "C...", "deployed_at": "<ledger>" }
```

Deploy sequence:
1. `stellar contract build`
2. `stellar contract deploy --network testnet --wasm target/...`
3. Record contract ID in `deployments/testnet.json`
4. Invoke one function to confirm liveness
5. Repeat for mainnet

## Testing approach

Use the Soroban in-process test environment — no network, no CLI, fast.

```rust
#[test]
fn test_full_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, SoroQuestEscrow);
    let client = SoroQuestEscrowClient::new(&env, &contract_id);
    // post → claim → approve → verify balances
}
```

Test auth separately with `env.mock_all_auths_allowing_non_root_auth()`
and verify `env.auths()` matches expected authorization trees.
