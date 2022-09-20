use std::time::{SystemTime, UNIX_EPOCH};

use crate::{sign_msg, Result};

const DELIMITER_SYMBOL: char = '.';

pub fn create_token(timestamp: u64, public_key: &String, private_key: &String) -> Result<String> {
    let mut msg = String::new();

    msg.push_str(&timestamp.to_string());
    msg.push(DELIMITER_SYMBOL);
    msg.push_str(public_key);

    let signature = sign_msg(&msg, private_key)?;

    msg.push(DELIMITER_SYMBOL);
    msg.push_str(signature.as_str());

    Ok(msg)
}

pub fn create_token_now(public_key: &String, private_key: &String) -> Result<String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    create_token(timestamp.as_secs(), public_key, private_key)
}
