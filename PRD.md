# PRD — soroquest-contracts

## What we are building

The on-chain layer of SoroQuest: a smart contract deployed on Stellar's
Soroban platform that holds bounty funds in escrow and releases them
automatically when work is approved. It is the source of truth for every
bounty on the platform. The frontend and indexer both derive their state
from this contract.

## Who it is for

**Primary: project maintainers and open source project owners**
People who need work done on their projects and are willing to pay for
it. They create bounties, fund them with USDC, and approve completed
work.

**Secondary: developers and contributors**
People who browse open bounties, claim ones they want to work on, and
receive USDC when their work is approved.

**Indirect: the SoroQuest indexer and frontend**
Both systems consume this contract's state and events. The contract's
public interface is effectively an API contract for the rest of the
system.

## What the product needs to do

### Core bounty lifecycle

1. **Post** — A project owner calls `post_bounty`, deposits USDC into
   the contract, and a new bounty is created with status `Open`.
2. **Claim** — A contributor calls `claim_bounty`, marking the bounty
   `Claimed` and reserving it. Only one contributor can hold a claim.
3. **Approve** — The owner calls `approve_completion`, releasing the
   full USDC amount to the claimant. Status becomes `Completed`.
4. **Cancel** — The owner calls `cancel_bounty`, returning USDC to
   themselves. Allowed when the bounty is `Open`, or when it is
   `Claimed` but the claim deadline has passed.

### Data the contract must store per bounty

- Unique ID (auto-incremented)
- Owner address
- Title and description
- USDC amount (deposited at post time, held until release or cancel)
- Token contract address (always USDC SAC address)
- Status: Open | Claimed | Completed | Cancelled
- Claimant address (set on claim, null before)
- Ledger sequence at creation
- Optional claim deadline (ledger sequence, 0 = no deadline)

### Authorization the contract must enforce

- Only the depositing owner can approve or cancel their bounty
- Only one contributor can claim an open bounty
- A claimed bounty cannot be cancelled unless the deadline has passed
- No function can be called without the appropriate `require_auth`

### Events the contract must emit

Every state change must emit a Soroban event so the indexer can track
the full history without re-reading all storage:

- `bounty_posted` — id, owner, amount, token, deadline
- `bounty_claimed` — id, claimant
- `bounty_completed` — id, claimant, amount
- `bounty_cancelled` — id, owner, amount

### What the contract must NOT do

- Never hold funds longer than necessary — release or return immediately
  on the triggering call
- Never accept tokens other than the specified token address per bounty
- Never allow partial payments or split payouts
- Never expose admin functions or upgrade paths in v1 — keep it simple
  and auditable

## Acceptance criteria

- All four lifecycle functions work end to end on testnet
- All authorization rules are enforced and tested
- All events are emitted and visible on Stellar Expert
- Contract is deployed and verified on both testnet and mainnet
- `cargo test` passes with zero failures
- No `unwrap()` or `panic!` in production code paths
