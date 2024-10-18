use near_bigint::U256;
use near_groth16_verifier::{Proof, Verifier};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, serde_json::json, Gas, PanicOnDefault, Promise};
use serde_json_canonicalizer::to_string as to_canonical_json;

mod models;
mod utils;

use models::{DerivationPath, LastLoginSession, Meta, SignRequest};
use utils::verify_lastlogin_token;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub verifier: Verifier, // because their is a proof to be verified (that you are who you are)
    signer_contract_id: AccountId, // mpc signer contract
    sessions: UnorderedMap<String, LastLoginSession>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(verifier: Verifier, signer_contract_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            verifier,
            signer_contract_id,
            sessions: UnorderedMap::new(b"s"),
        }
    }

    pub fn reinitialize(&mut self, verifier: Verifier, signer_contract_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract account can reinitialize"
        );

        self.verifier = verifier;
        self.signer_contract_id = signer_contract_id;
    }

    #[payable]
    pub fn sign_with_lastlogin_session(
        &mut self,
        proof: Proof,             // generated on your device
        public_inputs: Vec<U256>, // this accompanies the proof
        session_id: String,       // when was the last time you logged on?
        hostname: String,         // what server is calling
        chain: u64,
    ) -> Promise {

        // verify LastLogin session
        let session = verify_lastlogin_token(&self, &session_id, &hostname)
            .expect("Failed to verify LastLogin session"); // if this fails, then they will have to create_session

        // verify proof
        let verification_result = self.verifier.verify(public_inputs.clone(), proof);
        assert!(verification_result, "Verification failed");

        // prepare signature 

        // todo: this will need to be updated for a lastlogin payload, what will mpc expect?
        let mut payload = [0u8; 32];
        for (i, &value) in public_inputs[0..32].iter().enumerate() {
            payload[i] = U256::as_u32(&value) as u8;
        }

        let path = DerivationPath {
            chain,
            meta: Meta {
                // how important is this?
                email: session.email,
                user_id: session.session_id, // Using session_id as user_id
            },
        };

        let canonical_path =
            to_canonical_json(&path).expect("Failed to serialize path to canonical JSON");

        let request = SignRequest {
            payload,
            path: canonical_path,
            key_version: 0,
        };

        let args = json!({
            "request": request
        });

        let deposit = env::attached_deposit();
        const TGAS: u64 = 1_000_000_000_000;
        const GAS_FOR_MPC_CALL: Gas = Gas(100 * TGAS);

        Promise::new(self.signer_contract_id.clone()).function_call(
            "sign".to_string(),
            serde_json::to_vec(&args).unwrap(),
            deposit,
            GAS_FOR_MPC_CALL,
        )
    }

    pub fn create_session(&mut self, email: String, hostname: String) -> String {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract account can create_sessions"
        );

        // create session id with expiry
        let session_id =
            env::sha256(format!("{}{}{}", email, hostname, env::block_timestamp()).as_bytes());
        let session_id = hex::encode(session_id);
        let expires_at = env::block_timestamp() + 14 * 24 * 60 * 60 * 1_000_000_000; // 14 days from now

        let session = LastLoginSession {
            email,
            session_id: session_id.clone(),
            hostname,
            expires_at,
        };

        // save session
        self.sessions.insert(&session_id, &session);

        session_id
    }

    pub fn delete_session(&mut self, session_id: String) {
        self.sessions.remove(&session_id);
    }

    pub fn get_session(&self, session_id: String) -> Option<LastLoginSession> {
        self.sessions.get(&session_id)
    }
}
