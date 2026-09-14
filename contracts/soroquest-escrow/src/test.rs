#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{
        Client as TokenClient,
        StellarAssetClient as TokenAdminClient,
    },
    Address, Env, String,
};

fn create_token<'a>(env: &'a Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let token_addr = env.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(env, &token_addr.address()),
        TokenAdminClient::new(env, &token_addr.address()),
    )
}

fn setup() -> (Env, SoroQuestEscrowClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, SoroQuestEscrow);
    let client = SoroQuestEscrowClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_post_bounty() {
    // TODO: test that post_bounty creates a bounty and holds funds in escrow
    todo!("implement test_post_bounty");
}

#[test]
fn test_claim_bounty() {
    // TODO: test that claim_bounty marks bounty as Claimed
    todo!("implement test_claim_bounty");
}

#[test]
fn test_full_lifecycle() {
    // TODO: post → claim → approve → verify balances
    todo!("implement test_full_lifecycle");
}

#[test]
fn test_cancel_open_bounty() {
    // TODO: post → cancel → verify funds returned
    todo!("implement test_cancel_open_bounty");
}

#[test]
fn test_cancel_claimed_past_deadline() {
    // TODO: post → claim → advance ledger past deadline → cancel
    todo!("implement test_cancel_claimed_past_deadline");
}

#[test]
fn test_errors() {
    // TODO: test all SoroQuestError variants
    todo!("implement test_errors");
}
