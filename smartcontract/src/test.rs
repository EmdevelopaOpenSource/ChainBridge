#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env};

#[test]
fn test_initialization() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ChainBridge);
    let client = ChainBridgeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);
}

#[test]
fn test_create_htlc() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ChainBridge);
    let client = ChainBridgeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);

    client.init(&admin);

    // Create hash lock (32 bytes)
    let secret = Bytes::from_slice(&env, &[1u8; 32]);
    let hash_lock = env.crypto().sha256(&secret);
    let time_lock = env.ledger().timestamp() + 86400; // 24 hours

    let htlc_id = client.create_htlc(&sender, &receiver, &1000, &hash_lock, &time_lock);

    let htlc = client.get_htlc(&htlc_id);
    assert_eq!(htlc.amount, 1000);
    assert_eq!(htlc.sender, sender);
    assert_eq!(htlc.receiver, receiver);
}

#[test]
fn test_claim_htlc_with_correct_secret() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ChainBridge);
    let client = ChainBridgeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);

    client.init(&admin);

    let secret = Bytes::from_slice(&env, &[1u8; 32]);
    let hash_lock = env.crypto().sha256(&secret);
    let time_lock = env.ledger().timestamp() + 86400;

    let htlc_id = client.create_htlc(&sender, &receiver, &1000, &hash_lock, &time_lock);

    // Claim with correct secret
    client.claim_htlc(&receiver, &htlc_id, &secret);

    let status = client.get_htlc_status(&htlc_id);
    assert_eq!(status, HTLCStatus::Claimed);
}
