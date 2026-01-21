#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let nft_contract = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    
    // TODO: Test initialization
}

#[test]
fn test_create_commitment() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    
    // TODO: Test commitment creation
}

#[test]
fn test_settle() {
    let e = Env::default();
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    
    // TODO: Test settlement
}

#[contract]
struct DummyTokenContract;

#[contractimpl]
impl DummyTokenContract {
    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        // record last transfer for assertions
        e.storage().set(&Symbol::short("last_transfer"), &(from, to, amount));
    }
}

#[contract]
struct DummyNFTContract;

#[contractimpl]
impl DummyNFTContract {
    pub fn settle(e: Env, token_id: u32) {
        e.storage().set(&Symbol::short("nft_settled"), &token_id);
    }
}

#[test]
fn test_settlement_flow() {
    let e = Env::default();

    // Register dummy contracts
    let token_id = e.register_contract(None, DummyTokenContract);
    let nft_id = e.register_contract(None, DummyNFTContract);

    // Register the core contract
    let core_id = e.register_contract(None, CommitmentCoreContract);

    // Build a commitment that has expired
    let owner = Address::generate(&e);
    let asset_address = Address::Contract(token_id.clone());
    let now: u64 = e.ledger().timestamp();
    let commitment = Commitment {
        commitment_id: String::from_str(&e, "c1"),
        owner: owner.clone(),
        nft_token_id: 1,
        rules: CommitmentRules {
            duration_days: 1,
            max_loss_percent: 10,
            commitment_type: String::from_str(&e, "safe"),
            early_exit_penalty: 5,
            min_fee_threshold: 0,
        },
        amount: 100,
        asset_address: asset_address.clone(),
        created_at: now - 1000,
        expires_at: now - 1,
        current_value: 150,
        status: String::from_str(&e, "active"),
    };

    // Store nft_contract address for core contract to call
    e.storage().set(&Symbol::short("nft_contract"), &Address::Contract(nft_id.clone()));

    // Store the commitment
    e.storage().set(&(Symbol::short("commitments"), String::from_str(&e, "c1")), &commitment);

    // Call settle directly
    CommitmentCoreContract::settle(e.clone(), String::from_str(&e, "c1"));

    // Verify commitment marked settled
    let stored: Option<Commitment> = e.storage().get(&(Symbol::short("commitments"), String::from_str(&e, "c1")));
    assert!(stored.is_some());
    let stored = stored.unwrap();
    assert_eq!(stored.status, String::from_str(&e, "settled"));

    // Verify token transfer recorded
    let last: Option<(Address, Address, i128)> = e.storage().get(&Symbol::short("last_transfer"));
    assert!(last.is_some());
    let (_from, to, amt) = last.unwrap();
    assert_eq!(to, owner);
    assert_eq!(amt, 150);

    // Verify NFT settle called
    let settled: Option<u32> = e.storage().get(&Symbol::short("nft_settled"));
    assert!(settled.is_some());
    assert_eq!(settled.unwrap(), 1);
}

#[test]
#[should_panic(expected = "Commitment not expired")]
fn test_settlement_not_expired() {
    let e = Env::default();

    // Register core contract
    let _core_id = e.register_contract(None, CommitmentCoreContract);

    // Build a commitment that is not yet expired
    let owner = Address::generate(&e);
    let now: u64 = e.ledger().timestamp();
    let commitment = Commitment {
        commitment_id: String::from_str(&e, "c2"),
        owner: owner.clone(),
        nft_token_id: 2,
        rules: CommitmentRules {
            duration_days: 10,
            max_loss_percent: 10,
            commitment_type: String::from_str(&e, "balanced"),
            early_exit_penalty: 5,
            min_fee_threshold: 0,
        },
        amount: 100,
        asset_address: Address::generate(&e),
        created_at: now,
        expires_at: now + 10_000,
        current_value: 0,
        status: String::from_str(&e, "active"),
    };

    // Store the commitment
    e.storage().set(&(Symbol::short("commitments"), String::from_str(&e, "c2")), &commitment);

    // Attempt to settle; should panic
    CommitmentCoreContract::settle(e.clone(), String::from_str(&e, "c2"));
}

