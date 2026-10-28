# ADR 0001: Escrow State Machine and Lifecycle Transitions

## Status
Accepted

## Context
A decentralized bounty escrow contract must mediate financial transactions between untrusted bounty creators (funders) and arbitrary contributors without introducing custodial risk, fund locking, or race conditions.

Existing Web3 bounty systems often suffer from two failure modes:
1. Contributors submit work, but the funder goes unresponsive, indefinitely locking the contributor out of compensation.
2. A funder locks tokens, a malicious actor immediately claims the bounty with no intention of doing the work, locking the funder's capital indefinitely.

## Decision
We implemented a strict, deterministic 4-state state machine:
- `Open` (Initial state upon creation with locked tokens).
- `Claimed` (Contributor claims exclusive intent with a ledger-based deadline).
- `Completed` (Funder approves completed work, releasing tokens immediately).
- `Cancelled` (Funder reclaims locked tokens under specified safety conditions).

### Deadline & Cancellation Invariant
To prevent fund hostage situations:
1. When a bounty is `Open`, the funder can cancel at any time and receive a 100% refund.
2. When a bounty is `Claimed`:
   - If `claim_deadline == 0` (no deadline set), the funder may cancel at any time.
   - If `claim_deadline > 0`, the funder **cannot** cancel until `current_ledger > claim_deadline`. Once the deadline elapses without approval, the funder can cancel and recover their escrowed capital.
3. Once `Completed` or `Cancelled`, the bounty is terminal. No subsequent state transitions are permitted.

## Consequences
- State transitions are non-reversible and deterministic.
- Contributors have guaranteed exclusivity for the duration of the agreed deadline.
- Funders are protected from permanent denial-of-service via timeout reclamation.
