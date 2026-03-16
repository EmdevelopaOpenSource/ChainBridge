use crate::types::{ChainProof, CrossChainSwap, SwapOrder, HTLC};
use soroban_sdk::{contracttype, Address, Bytes, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    HTLCCounter,
    HTLC(u64),
    OrderCounter,
    Order(u64),
    SwapCounter,
    Swap(u64),
    SupportedChain(u8),
}

// Admin Functions
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

// HTLC Functions
pub fn get_htlc_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::HTLCCounter)
        .unwrap_or(0)
}

pub fn increment_htlc_counter(env: &Env) -> u64 {
    let counter = get_htlc_counter(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::HTLCCounter, &counter);
    counter
}

pub fn read_htlc(env: &Env, htlc_id: u64) -> Option<HTLC> {
    env.storage().persistent().get(&DataKey::HTLC(htlc_id))
}

pub fn write_htlc(env: &Env, htlc_id: u64, htlc: &HTLC) {
    env.storage()
        .persistent()
        .set(&DataKey::HTLC(htlc_id), htlc);
}

// Order Functions
pub fn get_order_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::OrderCounter)
        .unwrap_or(0)
}

pub fn increment_order_counter(env: &Env) -> u64 {
    let counter = get_order_counter(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::OrderCounter, &counter);
    counter
}

pub fn read_order(env: &Env, order_id: u64) -> Option<SwapOrder> {
    env.storage().persistent().get(&DataKey::Order(order_id))
}

pub fn write_order(env: &Env, order_id: u64, order: &SwapOrder) {
    env.storage()
        .persistent()
        .set(&DataKey::Order(order_id), order);
}

// Swap Functions
pub fn get_swap_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::SwapCounter)
        .unwrap_or(0)
}

pub fn increment_swap_counter(env: &Env) -> u64 {
    let counter = get_swap_counter(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::SwapCounter, &counter);
    counter
}

pub fn read_swap(env: &Env, swap_id: u64) -> Option<CrossChainSwap> {
    env.storage().persistent().get(&DataKey::Swap(swap_id))
}

pub fn write_swap(env: &Env, swap_id: u64, swap: &CrossChainSwap) {
    env.storage()
        .persistent()
        .set(&DataKey::Swap(swap_id), swap);
}

// Chain Support Functions
pub fn is_chain_supported(env: &Env, chain_id: u8) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::SupportedChain(chain_id))
}

pub fn add_supported_chain(env: &Env, chain_id: u8) {
    env.storage()
        .persistent()
        .set(&DataKey::SupportedChain(chain_id), &true);
}
