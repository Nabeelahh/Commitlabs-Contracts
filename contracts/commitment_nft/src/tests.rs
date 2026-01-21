#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Events}, Address, Env, String, vec, IntoVal};

#[test]
fn test_mint_event() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentNFTContract);
    let client = CommitmentNFTContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_commitment");
    let asset_address = Address::generate(&e);

    client.mint(
        &owner,
        &commitment_id,
        &30,
        &10,
        &String::from_str(&e, "safe"),
        &1000,
        &asset_address,
    );

    let events = e.events().all();
    let last_event = events.last().unwrap();
    
    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Mint").into_val(&e), 0u32.into_val(&e), owner.into_val(&e)]
    );
    let data: (String, u64) = last_event.2.into_val(&e);
    assert_eq!(data, (commitment_id, e.ledger().timestamp()));
}

#[test]
fn test_transfer_event() {
    let e = Env::default();
    let from = Address::generate(&e);
    let to = Address::generate(&e);
    let contract_id = e.register_contract(None, CommitmentNFTContract);
    let client = CommitmentNFTContractClient::new(&e, &contract_id);

    client.transfer(&from, &to, &0);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Transfer").into_val(&e), from.into_val(&e), to.into_val(&e)]
    );
    let data: (u32, u64) = last_event.2.into_val(&e);
    assert_eq!(data, (0u32, e.ledger().timestamp()));
}

#[test]
fn test_settle_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, CommitmentNFTContract);
    let client = CommitmentNFTContractClient::new(&e, &contract_id);

    client.settle(&0);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Settle").into_val(&e), 0u32.into_val(&e)]
    );
    let data: u64 = last_event.2.into_val(&e);
    assert_eq!(data, e.ledger().timestamp());
}

