#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Events}, Address, Env, String, Map, vec, IntoVal};

#[test]
fn test_attest_event() {
    let e = Env::default();
    let verified_by = Address::generate(&e);
    let contract_id = e.register_contract(None, AttestationEngineContract);
    let client = AttestationEngineContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    let attestation_type = String::from_str(&e, "health_check");
    let data = Map::new(&e);

    client.attest(&commitment_id, &attestation_type, &data, &verified_by);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Attest").into_val(&e), commitment_id.into_val(&e), verified_by.into_val(&e)]
    );
    let event_data: (String, bool, u64) = last_event.2.into_val(&e);
    assert_eq!(event_data.0, attestation_type);
    assert_eq!(event_data.1, true);
}

#[test]
fn test_record_fees_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, AttestationEngineContract);
    let client = AttestationEngineContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.record_fees(&commitment_id, &100);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("FeeRec").into_val(&e), commitment_id.into_val(&e)]
    );
    let event_data: (i128, u64) = last_event.2.into_val(&e);
    assert_eq!(event_data.0, 100);
}

#[test]
fn test_record_drawdown_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, AttestationEngineContract);
    let client = AttestationEngineContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.record_drawdown(&commitment_id, &5);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("Drawdown").into_val(&e), commitment_id.into_val(&e)]
    );
    let event_data: (i128, u64) = last_event.2.into_val(&e);
    assert_eq!(event_data.0, 5);
}

#[test]
fn test_calculate_compliance_score_event() {
    let e = Env::default();
    let contract_id = e.register_contract(None, AttestationEngineContract);
    let client = AttestationEngineContractClient::new(&e, &contract_id);

    let commitment_id = String::from_str(&e, "test_id");
    client.calculate_compliance_score(&commitment_id);

    let events = e.events().all();
    let last_event = events.last().unwrap();

    assert_eq!(last_event.0, contract_id);
    assert_eq!(
        last_event.1,
        vec![&e, symbol_short!("ScoreUpd").into_val(&e), commitment_id.into_val(&e)]
    );
    let event_data: (u32, u64) = last_event.2.into_val(&e);
    assert_eq!(event_data.0, 100);
}

