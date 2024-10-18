use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meta {
    pub email: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct DerivationPath {
    pub chain: u64,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SignRequest {
    pub payload: [u8; 32],
    pub path: String,
    pub key_version: u32,
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LastLoginSession {
    pub email: String, // can this be account Id? or other id // identifier maybe
    pub session_id: String,
    pub hostname: String,
    pub expires_at: u64,
}