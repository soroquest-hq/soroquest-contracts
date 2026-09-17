#![no_std]

mod error;
mod events;
mod storage;

pub use error::SoroQuestError;
pub use storage::{Bounty, BountyStatus};

use soroban_sdk::{
    contract, contractimpl, token::Client as TokenClient, Address, Env, String, Vec,
};
use storage::{get_bounty, get_bounty_count, increment_bounty_count, save_bounty};

#[contract]
pub struct SoroQuestEscrow;

#[contractimpl]
impl SoroQuestEscrow {
    /// Post a new bounty. Transfers `amount` of `token` from `owner` into the contract.
    /// Returns the new bounty ID.
    pub fn post_bounty(
        env: Env,
        owner: Address,
        title: String,
        description: String,
        amount: i128,
        token: Address,
        claim_deadline: u64,
    ) -> Result<u64, SoroQuestError> {
        owner.require_auth();

        if amount <= 0 {
            return Err(SoroQuestError::InvalidAmount);
        }
        if claim_deadline != 0 && claim_deadline <= env.ledger().sequence() as u64 {
            return Err(SoroQuestError::InvalidDeadline);
        }

        // Transfer token (e.g. USDC) from owner into contract escrow.
        let token_client = TokenClient::new(&env, &token);
        token_client.transfer(&owner, &env.current_contract_address(), &amount);

        let id = increment_bounty_count(&env);
        let bounty = Bounty {
            id,
            owner: owner.clone(),
            title,
            description,
            amount,
            token: token.clone(),
            status: BountyStatus::Open,
            claimant: None,
            created_at: env.ledger().sequence() as u64,
            claim_deadline,
        };
        save_bounty(&env, &bounty);
        events::emit_bounty_posted(&env, id, &owner, amount, &token, claim_deadline);

        Ok(id)
    }

    /// Claim an open bounty. Marks it `Claimed` and records the claimant.
    pub fn claim_bounty(
        env: Env,
        claimant: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError> {
        claimant.require_auth();

        let mut bounty = get_bounty(&env, bounty_id)?;

        if bounty.status != BountyStatus::Open {
            return Err(SoroQuestError::BountyNotOpen);
        }
        // Owner cannot claim their own bounty.
        if bounty.owner == claimant {
            return Err(SoroQuestError::NotOwner);
        }

        bounty.status = BountyStatus::Claimed;
        bounty.claimant = Some(claimant.clone());
        save_bounty(&env, &bounty);
        events::emit_bounty_claimed(&env, bounty_id, &claimant);

        Ok(())
    }

    /// Approve completion. Releases escrowed funds to the claimant.
    pub fn approve_completion(
        env: Env,
        owner: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError> {
        owner.require_auth();

        let mut bounty = get_bounty(&env, bounty_id)?;

        if bounty.owner != owner {
            return Err(SoroQuestError::NotOwner);
        }
        if bounty.status != BountyStatus::Claimed {
            return Err(SoroQuestError::BountyNotClaimed);
        }

        let claimant = bounty.claimant.clone().unwrap();
        let token_client = TokenClient::new(&env, &bounty.token);
        token_client.transfer(&env.current_contract_address(), &claimant, &bounty.amount);

        bounty.status = BountyStatus::Completed;
        save_bounty(&env, &bounty);
        events::emit_bounty_completed(&env, bounty_id, &claimant, bounty.amount);

        Ok(())
    }

    /// Cancel a bounty. Returns escrowed funds to the owner.
    ///
    /// Allowed when:
    /// - Status is `Open` (always).
    /// - Status is `Claimed` and `claim_deadline` has passed.
    pub fn cancel_bounty(
        env: Env,
        owner: Address,
        bounty_id: u64,
    ) -> Result<(), SoroQuestError> {
        owner.require_auth();

        let mut bounty = get_bounty(&env, bounty_id)?;

        if bounty.owner != owner {
            return Err(SoroQuestError::NotOwner);
        }

        match bounty.status {
            BountyStatus::Open => {} // always cancellable
            BountyStatus::Claimed => {
                // Only cancellable once the deadline has passed.
                let deadline = bounty.claim_deadline;
                if deadline == 0 || env.ledger().sequence() as u64 <= deadline {
                    return Err(SoroQuestError::DeadlineNotPassed);
                }
            }
            _ => return Err(SoroQuestError::BountyNotOpen),
        }

        let token_client = TokenClient::new(&env, &bounty.token);
        token_client.transfer(&env.current_contract_address(), &owner, &bounty.amount);

        bounty.status = BountyStatus::Cancelled;
        bounty.claimant = None;
        save_bounty(&env, &bounty);
        events::emit_bounty_cancelled(&env, bounty_id, &owner, bounty.amount);

        Ok(())
    }

    /// Get a single bounty by ID.
    pub fn get_bounty(env: Env, bounty_id: u64) -> Result<Bounty, SoroQuestError> {
        get_bounty(&env, bounty_id)
    }

    /// List bounties with an optional status filter and pagination (offset/limit).
    ///
    /// NOTE: This iterates the full bounty range; callers should use small `limit` values.
    pub fn list_bounties(
        env: Env,
        status: Option<BountyStatus>,
        offset: u64,
        limit: u64,
    ) -> Vec<Bounty> {
        let limit = if limit > 50 { 50 } else { limit };
        let count = get_bounty_count(&env);
        let mut results = Vec::new(&env);
        let mut skipped = 0u64;

        for id in 1..=count {
            if let Ok(bounty) = get_bounty(&env, id) {
                let matches = match &status {
                    Some(s) => bounty.status == *s,
                    None => true,
                };
                if matches {
                    if skipped < offset {
                        skipped += 1;
                        continue;
                    }
                    if results.len() as u64 >= limit {
                        break;
                    }
                    results.push_back(bounty);
                }
            }
        }
        results
    }

    /// Get the total number of bounties ever posted.
    pub fn get_bounty_count(env: Env) -> u64 {
        get_bounty_count(&env)
    }
}

#[cfg(test)]
mod test;
