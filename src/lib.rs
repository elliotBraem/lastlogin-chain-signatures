use near_bigint::U256;
use near_groth16_verifier::{Proof, Verifier};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, serde_json::json, Gas, PanicOnDefault, Promise};
use serde_json_canonicalizer::to_string as to_canonical_json;

mod models;
mod utils;

use models::{DerivationPath, Meta, SignRequest};
use utils::{hash_to_u256, decode_jwt_payload};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub verifier: Verifier, // because their is a proof to be verified (that you are who you are)
    signer_contract_id: AccountId, // mpc signer contract
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(verifier: Verifier, signer_contract_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            verifier,
            signer_contract_id,
        }
    }

    pub fn reinitialize(&mut self, verifier: Verifier, signer_contract_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Only the contract account can reinitialize"
        );

        self.verifier = verifier; // if you need to update the verifier
        self.signer_contract_id = signer_contract_id;
    }

    #[payable]
    pub fn sign_with_lastlogin_session(
        &mut self,
        proof: Proof,             // generated on your device
        public_inputs: Vec<U256>, // accompanies the proof for verification
        challenge: String,        // to ensure freshness
        chain: u64,               // target chain for signature
    ) -> Promise {
        // verify proof
        let verification_result = self.verifier.verify(public_inputs.clone(), proof);
        assert!(verification_result, "Verification failed");

        let mut payload = [0u8; 32];
        for (i, &value) in public_inputs[68..100].iter().enumerate() {
            payload[i] = U256::as_u32(&value) as u8;
        }

        // verify challenge
        let challenge_hash = hash_to_u256(&challenge).expect("Failed to hash challenge");
        assert_eq!(
           &public_inputs[64..68], // what is our public_inputs structure?
            &challenge_hash,
            "Challenge mismatch"
        );

        let mut payload = [0u8; 32];
        for (i, &value) in public_inputs[68..100].iter().enumerate() {
            payload[i] = U256::as_u32(&value) as u8;
        }

        // validate jwt session
        let (_sub, email, _iss, aud, exp) =
            decode_jwt_payload(&challenge).expect("Failed to decode JWT payload");
        assert_eq!(
            exp > env::block_timestamp() / 1_000_000_000,
            true,
            "Token expired"
        );

        // prepare signature
        let path = DerivationPath {
            chain,
            meta: Meta { email, aud },
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

        // send to mpc for signature
        Promise::new(self.signer_contract_id.clone()).function_call(
            "sign".to_string(),
            serde_json::to_vec(&args).unwrap(),
            deposit,
            GAS_FOR_MPC_CALL,
        )
    }
}
