use soroban_sdk::{contracttype, Address, Env, String};
use crate::error::SoroQuestError;

/// Current lifecycle state of a bounty.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BountyStatus {
    Open,
    Claimed,
    Completed,
    Cancelled,
}

/// On-chain representation of a bounty.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Bounty {
    pub id: u64,
    pub owner: Address,
    pub title: String,
    pub description: String,
    pub amount: i128,
    pub token: Address,
    pub status: BountyStatus,
    pub claimant: Option<Address>,
    pub created_at: u64,
    pub claim_deadline: u64,
}

/// Persistent storage keys.
#[contracttype]
pub enum StorageKey {
    BountyCount,
    Bounty(u64),
}

/// ~1 year on Stellar mainnet (assuming 5s ledger close time).
const LEDGER_TTL_EXTENSION: u32 = 535_000;

// ---------------------------------------------------------------------------
// Storage helpers
// ---------------------------------------------------------------------------

pub fn get_bounty_count(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&StorageKey::BountyCount)
        .unwrap_or(0)
}

pub fn increment_bounty_count(env: &Env) -> u64 {
    let count = get_bounty_count(env) + 1;
    env.storage()
        .persistent()
        .set(&StorageKey::BountyCount, &count);
    env.storage()
        .persistent()
        .extend_ttl(&StorageKey::BountyCount, LEDGER_TTL_EXTENSION, LEDGER_TTL_EXTENSION);
    count
}

pub fn get_bounty(env: &Env, id: u64) -> Result<Bounty, SoroQuestError> {
    env.storage()
        .persistent()
        .get(&StorageKey::Bounty(id))
        .ok_or(SoroQuestError::BountyNotFound)
}

pub fn save_bounty(env: &Env, bounty: &Bounty) {
    env.storage()
        .persistent()
        .set(&StorageKey::Bounty(bounty.id), bounty);
    env.storage()
        .persistent()
        .extend_ttl(
            &StorageKey::Bounty(bounty.id),
            LEDGER_TTL_EXTENSION,
            LEDGER_TTL_EXTENSION,
        );
}
