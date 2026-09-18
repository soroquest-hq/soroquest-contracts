# Contract Reference

This document outlines the public interface of the SoroQuest Escrow smart contract.

## Data Types

### `BountyStatus` (Enum)
Represents the lifecycle state of a bounty.
- `Open`: Bounty is funded and available to be claimed.
- `Claimed`: A hunter has claimed the bounty and is working on it.
- `Completed`: Work was approved, and funds were released.
- `Cancelled`: Bounty was cancelled, and funds were returned to the owner.

### `SoroQuestError` (Enum)
- `1` `BountyNotFound`: The requested bounty ID does not exist.
- `2` `BountyNotOpen`: The action requires the bounty to be in the `Open` state.
- `3` `BountyNotClaimed`: The action requires the bounty to be in the `Claimed` state.
- `4` `AlreadyClaimed`: The bounty has already been claimed (unused directly, covered by `BountyNotOpen`).
- `5` `NotOwner`: The caller is not the owner of the bounty.
- `6` `DeadlineNotPassed`: Attempted to cancel a claimed bounty before its deadline.
- `7` `InvalidAmount`: The bounty amount must be greater than zero.
- `8` `InvalidDeadline`: The claim deadline must be in the future (or 0).
- `9` `TransferFailed`: Token transfer failed (handled natively by the SAC interface panicking, so this is rarely returned directly).

---

## Public Functions

### `post_bounty`
Posts a new bounty and escrows the funds.

**Parameters:**
- `owner: Address` — The creator of the bounty. Must authorize the call.
- `title: String` — Short title of the task.
- `description: String` — Detailed description.
- `amount: i128` — Reward amount in the token's smallest unit (e.g., stroops).
- `token_address: Address` — The contract address of the token (e.g., USDC or any Stellar Asset).
- `claim_deadline: u64` — Ledger sequence after which the owner can unilaterally cancel a claimed bounty. Set to `0` for no deadline.

**Returns:** `Result<u64, SoroQuestError>` — The new bounty ID.

**Example invocation:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER_SECRET> \
  -- network testnet \
  -- \
  post_bounty \
  --owner <OWNER_ADDRESS> \
  --title "Fix bug" \
  --description "Fix the login bug" \
  --amount 10000000 \
  --token_address <USDC_CONTRACT_ID> \
  --claim_deadline 0
```

---

### `claim_bounty`
Claims an open bounty.

**Parameters:**
- `claimant: Address` — The user claiming the bounty. Must authorize the call.
- `bounty_id: u64` — The ID of the bounty.

**Returns:** `Result<(), SoroQuestError>`

**Example invocation:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <CLAIMANT_SECRET> \
  -- network testnet \
  -- \
  claim_bounty \
  --claimant <CLAIMANT_ADDRESS> \
  --bounty_id 1
```

---

### `approve_completion`
Approves the work and releases escrowed funds to the claimant.

**Parameters:**
- `owner: Address` — The owner of the bounty. Must authorize the call.
- `bounty_id: u64` — The ID of the bounty.

**Returns:** `Result<(), SoroQuestError>`

**Example invocation:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER_SECRET> \
  -- network testnet \
  -- \
  approve_completion \
  --owner <OWNER_ADDRESS> \
  --bounty_id 1
```

---

### `cancel_bounty`
Cancels the bounty and returns funds to the owner.

**Parameters:**
- `owner: Address` — The owner of the bounty. Must authorize the call.
- `bounty_id: u64` — The ID of the bounty.

**Returns:** `Result<(), SoroQuestError>`

**Example invocation:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <OWNER_SECRET> \
  -- network testnet \
  -- \
  cancel_bounty \
  --owner <OWNER_ADDRESS> \
  --bounty_id 1
```

---

### `get_bounty`
Retrieves a single bounty by ID. Read-only.

**Parameters:**
- `bounty_id: u64` — The ID of the bounty.

**Returns:** `Result<Bounty, SoroQuestError>`

---

### `list_bounties`
Lists bounties with pagination and filtering. Read-only.

**Parameters:**
- `status: Option<BountyStatus>` — Optional status filter (e.g., limit to only `Open` bounties).
- `offset: u64` — Number of matching bounties to skip.
- `limit: u64` — Maximum number of bounties to return (capped at 50).

**Returns:** `Vec<Bounty>`

---

### `get_bounty_count`
Gets the total number of bounties ever created. Read-only.

**Parameters:** None

**Returns:** `u64`
