use std::num::ParseIntError;
use std::string::FromUtf8Error;
use std::time::{SystemTime, UNIX_EPOCH};
use openssl::rsa::{Padding, Rsa};
use thiserror::Error;

const RSA_KEY_SIZE: usize = 2048;

pub fn generate_keys() -> Result<(String, String), openssl::ssl::Error> {
    let keys = Rsa::generate(RSA_KEY_SIZE as u32)?;
    let private_key = String::from_utf8(keys.private_key_to_pem()?).unwrap();
    let public_key = String::from_utf8(keys.public_key_to_pem()?).unwrap();
    Ok((private_key, public_key))
}

fn decrypt_signature(pub_key: String, signature: String) -> Result<Vec<u8>, openssl::ssl::Error> {
    let key = Rsa::public_key_from_pem(&pub_key.as_bytes())?;

    let mut buff: [u8; RSA_KEY_SIZE / 8] = [0; RSA_KEY_SIZE / 8];

    key.public_decrypt(signature.as_bytes(), &mut buff, Padding::PKCS1)?;

    Ok(buff[0 .. 8].to_vec())
}

#[derive(Error, Debug)]
pub enum ValidateSignError {
    #[error("failed to decrypt signature {0}")]
    InvalidSignature(#[from] openssl::ssl::Error),
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

    let timestamp = String::from_utf8(decrypted)?
        .parse::<u64>()?;

    Ok((now - timestamp) < VALID_INTERVAL)
}