extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    token::{Client as TokenClient, StellarAssetClient as TokenAdminClient},
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
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Fix bug"),
        &String::from_str(&env, "Fix it fast"),
        &500,
        &token.address,
        &0,
    );

    assert_eq!(id, 1);
    assert_eq!(token.balance(&owner), 500);
    assert_eq!(token.balance(&client.address), 500);

    let bounty = client.get_bounty(&1);
    assert_eq!(bounty.owner, owner);
    assert_eq!(bounty.status, BountyStatus::Open);
}

#[test]
fn test_full_lifecycle() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    // Post
    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Build feature"),
        &String::from_str(&env, "Build it"),
        &1000,
        &token.address,
        &0,
    );

    assert_eq!(client.get_bounty(&id).status, BountyStatus::Open);
    assert_eq!(token.balance(&client.address), 1000);

    // Claim
    client.claim_bounty(&claimant, &id);
    let bounty = client.get_bounty(&id);
    assert_eq!(bounty.status, BountyStatus::Claimed);
    assert_eq!(bounty.claimant, Some(claimant.clone()));

    // Approve
    client.approve_completion(&owner, &id);
    assert_eq!(client.get_bounty(&id).status, BountyStatus::Completed);

    // Verify balances
    assert_eq!(token.balance(&claimant), 1000);
    assert_eq!(token.balance(&client.address), 0);
}

#[test]
fn test_cancel_open_bounty() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );

    client.cancel_bounty(&owner, &id);

    let bounty = client.get_bounty(&id);
    assert_eq!(bounty.status, BountyStatus::Cancelled);
    assert_eq!(token.balance(&owner), 1000);
    assert_eq!(token.balance(&client.address), 0);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")] // DeadlineNotPassed = 6
fn test_cancel_claimed_before_deadline() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    env.ledger().with_mut(|li| li.sequence_number = 100);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &200, // Deadline is ledger 200
    );

    client.claim_bounty(&claimant, &id);

    env.ledger().with_mut(|li| li.sequence_number = 150); // Before deadline

    client.cancel_bounty(&owner, &id);
}

#[test]
fn test_cancel_claimed_after_deadline() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    env.ledger().with_mut(|li| li.sequence_number = 100);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &200, // Deadline is ledger 200
    );

    client.claim_bounty(&claimant, &id);

    env.ledger().with_mut(|li| li.sequence_number = 250); // After deadline

    client.cancel_bounty(&owner, &id);

    let bounty = client.get_bounty(&id);
    assert_eq!(bounty.status, BountyStatus::Cancelled);
    assert_eq!(token.balance(&owner), 1000);
    assert_eq!(bounty.claimant, None);
}

#[test]
fn test_cancel_claimed_no_deadline() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0, // No deadline
    );

    client.claim_bounty(&claimant, &id);
    client.cancel_bounty(&owner, &id);

    assert_eq!(client.get_bounty(&id).status, BountyStatus::Cancelled);
    assert_eq!(token.balance(&owner), 1000);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")] // NotOwner = 5
fn test_approve_non_owner() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let other = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );
    client.claim_bounty(&claimant, &id);

    client.approve_completion(&other, &id); // Should panic
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")] // NotOwner = 5
fn test_cancel_non_owner() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );

    client.cancel_bounty(&other, &id); // Should panic
}

#[test]
fn test_events() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &2000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );

    let post_events: std::vec::Vec<_> = env
        .events()
        .all()
        .into_iter()
        .filter(|e| e.0 == client.address)
        .collect();
    assert_eq!(post_events.len(), 1);

    client.claim_bounty(&claimant, &id);
    let claim_events: std::vec::Vec<_> = env
        .events()
        .all()
        .into_iter()
        .filter(|e| e.0 == client.address)
        .collect();
    assert_eq!(claim_events.len(), 2);

    client.approve_completion(&owner, &id);
    let approve_events: std::vec::Vec<_> = env
        .events()
        .all()
        .into_iter()
        .filter(|e| e.0 == client.address)
        .collect();
    assert_eq!(approve_events.len(), 3);

    let id2 = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task 2"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );
    client.cancel_bounty(&owner, &id2);

    let cancel_events: std::vec::Vec<_> = env
        .events()
        .all()
        .into_iter()
        .filter(|e| e.0 == client.address)
        .collect();
    assert_eq!(cancel_events.len(), 5);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")] // InvalidAmount = 7
fn test_post_zero_amount() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (token, _) = create_token(&env, &admin);

    client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &0,
        &token.address,
        &0,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")] // BountyNotOpen = 2
fn test_claim_already_claimed() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant1 = Address::generate(&env);
    let claimant2 = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );

    client.claim_bounty(&claimant1, &id);
    client.claim_bounty(&claimant2, &id); // Should panic
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")] // BountyNotClaimed = 3
fn test_approve_cancelled_bounty() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &1000);

    let id = client.post_bounty(
        &owner,
        &String::from_str(&env, "Task"),
        &String::from_str(&env, "Desc"),
        &1000,
        &token.address,
        &0,
    );

    client.cancel_bounty(&owner, &id);
    client.approve_completion(&owner, &id); // Should panic
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // BountyNotFound = 1
fn test_get_nonexistent_bounty() {
    let (_, client) = setup();
    client.get_bounty(&999);
}

#[test]
fn test_list_bounties_limit() {
    let (env, client) = setup();
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (token, token_admin) = create_token(&env, &admin);

    token_admin.mint(&owner, &6000);

    for _ in 0..55 {
        client.post_bounty(
            &owner,
            &String::from_str(&env, "Task"),
            &String::from_str(&env, "Desc"),
            &100,
            &token.address,
            &0,
        );
    }

    let results = client.list_bounties(&None, &0, &100);
    assert_eq!(results.len(), 50);
}

#[test]
fn test_multi_token_lifecycle() {
    let (env, client) = setup();
    let admin_a = Address::generate(&env);
    let admin_b = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant_a = Address::generate(&env);
    let claimant_b = Address::generate(&env);

    let (token_a, token_admin_a) = create_token(&env, &admin_a);
    let (token_b, token_admin_b) = create_token(&env, &admin_b);

    token_admin_a.mint(&owner, &1000);
    token_admin_b.mint(&owner, &2500);

    let id_a = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty in Token A"),
        &String::from_str(&env, "First token reward"),
        &1000,
        &token_a.address,
        &0,
    );

    let id_b = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty in Token B"),
        &String::from_str(&env, "Second token reward"),
        &2500,
        &token_b.address,
        &0,
    );

    let bounty_a = client.get_bounty(&id_a);
    let bounty_b = client.get_bounty(&id_b);

    assert_eq!(bounty_a.token_address, token_a.address);
    assert_eq!(bounty_b.token_address, token_b.address);
    assert_eq!(token_a.balance(&client.address), 1000);
    assert_eq!(token_b.balance(&client.address), 2500);
    assert_eq!(token_a.balance(&owner), 0);
    assert_eq!(token_b.balance(&owner), 0);

    client.claim_bounty(&claimant_a, &id_a);
    client.claim_bounty(&claimant_b, &id_b);

    client.approve_completion(&owner, &id_a);
    assert_eq!(token_a.balance(&claimant_a), 1000);
    assert_eq!(token_b.balance(&claimant_a), 0);
    assert_eq!(token_a.balance(&client.address), 0);

    client.approve_completion(&owner, &id_b);
    assert_eq!(token_b.balance(&claimant_b), 2500);
    assert_eq!(token_a.balance(&claimant_b), 0);
    assert_eq!(token_b.balance(&client.address), 0);

    assert_eq!(client.get_bounty(&id_a).status, BountyStatus::Completed);
    assert_eq!(client.get_bounty(&id_b).status, BountyStatus::Completed);
}

#[test]
fn test_multi_token_cancellation_refund() {
    let (env, client) = setup();
    let admin_a = Address::generate(&env);
    let admin_b = Address::generate(&env);
    let owner = Address::generate(&env);

    let (token_a, token_admin_a) = create_token(&env, &admin_a);
    let (token_b, token_admin_b) = create_token(&env, &admin_b);

    token_admin_a.mint(&owner, &1500);
    token_admin_b.mint(&owner, &3000);

    let id_a = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty A"),
        &String::from_str(&env, "Refund Token A test"),
        &1500,
        &token_a.address,
        &0,
    );

    let id_b = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty B"),
        &String::from_str(&env, "Refund Token B test"),
        &3000,
        &token_b.address,
        &0,
    );

    client.cancel_bounty(&owner, &id_a);
    assert_eq!(token_a.balance(&owner), 1500);
    assert_eq!(token_a.balance(&client.address), 0);
    assert_eq!(token_b.balance(&owner), 0);
    assert_eq!(token_b.balance(&client.address), 3000);

    client.cancel_bounty(&owner, &id_b);
    assert_eq!(token_b.balance(&owner), 3000);
    assert_eq!(token_b.balance(&client.address), 0);

    assert_eq!(client.get_bounty(&id_a).status, BountyStatus::Cancelled);
    assert_eq!(client.get_bounty(&id_b).status, BountyStatus::Cancelled);
}

#[test]
fn test_multi_token_claimed_cancellation_with_deadlines() {
    let (env, client) = setup();
    let admin_a = Address::generate(&env);
    let admin_b = Address::generate(&env);
    let owner = Address::generate(&env);
    let claimant_a = Address::generate(&env);
    let claimant_b = Address::generate(&env);

    let (token_a, token_admin_a) = create_token(&env, &admin_a);
    let (token_b, token_admin_b) = create_token(&env, &admin_b);

    token_admin_a.mint(&owner, &1000);
    token_admin_b.mint(&owner, &2000);

    env.ledger().with_mut(|li| li.sequence_number = 100);

    let id_a = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty A"),
        &String::from_str(&env, "Deadline 200"),
        &1000,
        &token_a.address,
        &200,
    );

    let id_b = client.post_bounty(
        &owner,
        &String::from_str(&env, "Bounty B"),
        &String::from_str(&env, "Deadline 300"),
        &2000,
        &token_b.address,
        &300,
    );

    client.claim_bounty(&claimant_a, &id_a);
    client.claim_bounty(&claimant_b, &id_b);

    env.ledger().with_mut(|li| li.sequence_number = 250);

    client.cancel_bounty(&owner, &id_a);
    assert_eq!(token_a.balance(&owner), 1000);
    assert_eq!(token_a.balance(&client.address), 0);
    assert_eq!(token_b.balance(&client.address), 2000);

    env.ledger().with_mut(|li| li.sequence_number = 350);

    client.cancel_bounty(&owner, &id_b);
    assert_eq!(token_b.balance(&owner), 2000);
    assert_eq!(token_b.balance(&client.address), 0);
}
