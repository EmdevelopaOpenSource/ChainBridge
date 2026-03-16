use soroban_sdk::{contracttype, Address, Bytes, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HTLCStatus {
    Active,
    Claimed,
    Refunded,
    Expired,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Chain {
    Bitcoin,
    Ethereum,
    Solana,
    Polygon,
    BSC,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HTLC {
    pub sender: Address,
    pub receiver: Address,
    pub amount: i128,
    pub hash_lock: Bytes, // SHA256 hash of secret
    pub time_lock: u64,   // Expiry timestamp
    pub status: HTLCStatus,
    pub secret: Option<Bytes>, // Revealed secret (if claimed)
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapOrder {
    pub id: u64,
    pub creator: Address,
    pub from_chain: Chain,
    pub to_chain: Chain,
    pub from_asset: String,
    pub to_asset: String,
    pub from_amount: i128,
    pub to_amount: i128,
    pub expiry: u64,
    pub matched: bool,
    pub counterparty: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainSwap {
    pub id: u64,
    pub stellar_htlc_id: u64,
    pub other_chain: Chain,
    pub other_chain_tx: String, // Transaction hash on other chain
    pub stellar_party: Address,
    pub other_party: String, // Address on other chain
    pub completed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChainProof {
    pub chain: Chain,
    pub tx_hash: String,
    pub block_height: u64,
    pub proof_data: Bytes, // Merkle proof or SPV proof
}
