use soroban_sdk::{symbol_short, Address, Env};

/// Emitted when a new bounty is posted and funds enter escrow.
pub fn emit_bounty_posted(
    env: &Env,
    bounty_id: u64,
    owner: &Address,
    amount: i128,
    token: &Address,
    claim_deadline: u64,
) {
    env.events().publish(
        (symbol_short!("posted"), bounty_id),
        (owner.clone(), amount, token.clone(), claim_deadline),
    );
}

/// Emitted when a hunter claims an open bounty.
pub fn emit_bounty_claimed(env: &Env, bounty_id: u64, claimant: &Address) {
    env.events()
        .publish((symbol_short!("claimed"), bounty_id), claimant.clone());
}

/// Emitted when the owner approves completion and funds are released to the claimant.
pub fn emit_bounty_completed(env: &Env, bounty_id: u64, claimant: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("completed"), bounty_id),
        (claimant.clone(), amount),
    );
}

/// Emitted when the owner cancels a bounty and funds are returned.
pub fn emit_bounty_cancelled(env: &Env, bounty_id: u64, owner: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("cancelled"), bounty_id),
        (owner.clone(), amount),
    );
}
