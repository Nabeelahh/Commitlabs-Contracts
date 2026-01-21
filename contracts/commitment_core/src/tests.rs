#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Events}, Address, Env, String, vec, IntoVal};

#[test]
fn test_create_commitment_event() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&e, &contract_id);

    let rules = CommitmentRules {
        duration_days: 30,
        max_loss_percent: 10,
        commitment_type: String::from_str(&e, "safe"),
        early_exit_penalty: 5,
        min_fee_threshold: 100,
    };

    client.create_commitment(&owner, &1000, &Address::generate(&e), &rules);

    let events = e.events().all();
    let last_event = events.last().unwrap();
    
    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Created").into_val(&e), String::from_str(&e, "commitment_id_placeholder").into_val(&e), owner.into_val(&e)]
    );
    let data: (i128, CommitmentRules, u32, u64) = last_event.2.into_val(&e);
    assert_eq!(data.0, 1000);
    assert_eq!(data.1, rules);
    assert_eq!(data.2, 0u32);
}

#[test]
fn test_update_value_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.update_value(&commitment_id, &1100);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("ValUpd").into_val(&e), commitment_id.into_val(&e)]
    );
    let data: (i128, u64) = last_event.2.into_val(&e);
    assert_eq!(data.0, 1100);
}

#[test]
fn test_settle_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.settle(&commitment_id);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Settled").into_val(&e), commitment_id.into_val(&e)]
    );
    let data: (i128, u64) = last_event.2.into_val(&e);
    assert_eq!(data.0, 0);
}

#[test]
fn test_early_exit_event() {
    let e = Env::default();
    let caller = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.early_exit(&commitment_id, &caller);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("EarlyExt").into_val(&e), commitment_id.into_val(&e), caller.into_val(&e)]
    );
    let data: (i128, i128, u64) = last_event.2.into_val(&e);
    assert_eq!(data.0, 0);
    assert_eq!(data.1, 0);
}

#[test]
fn test_allocate_event() {
    let e = Env::default();
    let target_pool = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentCoreContract);
    let client = CommitmentCoreContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.allocate(&commitment_id, &target_pool, &500);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Alloc").into_val(&e), commitment_id.into_val(&e), target_pool.into_val(&e)]
    );
    let data: (i128, u64) = last_event.2.into_val(&e);
    assert_eq!(data.0, 500);
}

