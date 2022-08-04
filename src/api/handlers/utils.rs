use openssl::rsa::{Padding, Rsa};
use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

const RSA_KEY_SIZE: usize = 512;

pub fn generate_keys() -> Result<(String, String), openssl::ssl::Error> {
    let keys = Rsa::generate(RSA_KEY_SIZE as u32)?;
    let private_key = hex::encode(keys.private_key_to_der()?);
    let public_key = hex::encode(keys.public_key_to_der()?);
    Ok((private_key, public_key))
}

#[derive(Error, Debug)]
pub enum DecryptError {
    #[error("failed to decode: {0}")]
    Decode(#[from] hex::FromHexError),
    #[error("failed to decrypt: {0}")]
    Decrypt(#[from] openssl::error::ErrorStack),
}

fn decrypt_signature(pub_key: String, signature: String) -> Result<Vec<u8>, DecryptError> {
    let key = Rsa::public_key_from_der(
        hex::decode(pub_key)
            .map_err(DecryptError::Decode)?
            .as_slice(),
    )
    .map_err(DecryptError::Decrypt)?;

    let mut buff: [u8; RSA_KEY_SIZE / 8] = [0; RSA_KEY_SIZE / 8];

    key.public_decrypt(signature.as_bytes(), &mut buff, Padding::PKCS1)?;

    Ok(buff[0..8].to_vec())
}

#[derive(Error, Debug)]
pub enum ValidateSignError {
    #[error("failed to decrypt signature {0}")]
    InvalidSignature(#[from] DecryptError),
    #[error("failed to parse decrypted timestamp")]
    FailedToParseDecrypted(#[from] FromUtf8Error),
    #[error("failed to parse timestamp")]
    FailedToParseFromU64(#[from] ParseIntError),
}

const VALID_INTERVAL: u64 = 60 * 60;

pub fn check_signature(pub_key: String, signature: String) -> Result<bool, ValidateSignError> {
    let decrypted = decrypt_signature(pub_key, signature)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let timestamp = String::from_utf8(decrypted)?.parse::<u64>()?;

    Ok((now - timestamp) < VALID_INTERVAL)
}
