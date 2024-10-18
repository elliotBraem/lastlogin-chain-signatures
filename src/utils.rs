use near_sdk::env;
use std::error::Error;

use crate::models::LastLoginSession;
use crate::Contract;

pub fn verify_lastlogin_token(
    contract: &Contract,
    session_id: &str,
    hostname: &str,
) -> Result<LastLoginSession, Box<dyn Error>> {
    assert!(
        is_session_valid(contract, session_id.to_string(), hostname.to_string()),
        "Invalid or expired session"
    );

    let session = contract
        .get_session(session_id.to_string())
        .ok_or("Session not found")?;

    // verify the hostname
    if session.hostname != hostname {
        return Err("Hostname mismatch".into());
    }

    // check if expired
    if session.expires_at < env::block_timestamp() {
        return Err("Session has expired".into());
    }

    Ok(session)
}

pub fn is_session_valid(contract: &Contract, session_id: String, hostname: String) -> bool {
    if let Some(session) = contract.sessions.get(&session_id) {
        session.hostname == hostname && session.expires_at > env::block_timestamp()
    } else {
        false
    }
}
