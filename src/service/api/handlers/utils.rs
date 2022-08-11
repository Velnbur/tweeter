use axum::headers::authorization::Credentials;
use ecdsa::signature::Verifier;
use elliptic_curve::rand_core::OsRng;
use k256::ecdsa::{Signature, SigningKey, VerifyingKey};

use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub struct PubKey(pub String);

impl Credentials for PubKey {
    const SCHEME: &'static str = "Public Key";

    fn decode(value: &axum::http::HeaderValue) -> Option<Self> {
        if value.is_empty() {
            return None;
        }
        let inner = match value.to_str() {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(Self(String::from(inner)))
    }

    fn encode(&self) -> axum::http::HeaderValue {
        todo!()
    }
}

pub fn generate_keys() -> (String, String) {
    let keys = SigningKey::random(&mut OsRng);

    (
        base64::encode(keys.to_bytes()),
        base64::encode(keys.verifying_key().to_bytes()),
    )
}

use thiserror::Error;

use crate::records::tweets::Tweet;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("failed to decode: {0}")]
    DecodeError(#[from] base64::DecodeError),
    #[error("failed to create verifying key: {0}")]
    VerifyKeyError(ecdsa::Error),
    #[error("failed to verify: {0}")]
    VerifyingError(signature::Error),
}

pub fn verify_signature(msg: &String, sign: &String, pub_key: &String) -> Result<(), VerifyError> {
    let key = VerifyingKey::from_sec1_bytes(
        base64::decode(pub_key)
            .map_err(VerifyError::DecodeError)?
            .as_slice(),
    )
    .map_err(VerifyError::VerifyKeyError)?;

    let sign = Signature::from_str(sign.as_str()).map_err(VerifyError::VerifyingError)?;

    key.verify(msg.as_bytes(), &sign)
        .map_err(VerifyError::VerifyingError)
}

pub fn verify_tweet(tweet: &Tweet) -> Result<(), VerifyError> {
    let mut msg = String::new();

    msg.push_str(&tweet.title.to_owned());
    msg.push('\n');
    msg.push_str(&tweet.description.to_owned());
    msg.push('\n');
    msg.push_str(&tweet.timestamp.to_string());
    msg.push('\n');
    msg.push_str(&tweet.user_id.to_owned());

    verify_signature(&msg, &tweet.signature, &tweet.user_id)
}

#[cfg(test)]
mod test {
    use k256::ecdsa::{Signature, SigningKey};
    use signature::Signer;

    use crate::records::tweets::Tweet;

    use super::{generate_keys, verify_signature, verify_tweet};

    #[test]
    fn test_verify_signature() {
        let (priv_key, pub_key) = generate_keys();

        let sign_key =
            SigningKey::from_bytes(base64::decode(priv_key).unwrap().as_slice()).unwrap();

        let msg = "hello, world";
        let sign: Signature = sign_key.sign(msg.as_bytes());

        verify_signature(&msg.to_string(), &sign.to_string(), &pub_key).unwrap();
    }

    #[test]
    fn test_verify_tweet() {
        let (priv_key, pub_key) = generate_keys();

        let sign_key =
            SigningKey::from_bytes(base64::decode(priv_key).unwrap().as_slice()).unwrap();

        let title = "title";
        let description = "description";
        let timestamp = 123123;

        let mut msg = String::new();

        msg.push_str(&title.to_owned());
        msg.push('\n');
        msg.push_str(&description.to_owned());
        msg.push('\n');
        msg.push_str(&timestamp.to_string());
        msg.push('\n');
        msg.push_str(&pub_key.to_owned());

        let sign: Signature = sign_key.sign(msg.as_bytes());

        let tweet = Tweet {
            id: 0,
            title: title.to_string(),
            description: description.to_string(),
            timestamp,
            user_id: pub_key.clone(),
            signature: sign.to_string(),
            hash: None,
            prev_id: None,
        };

        verify_tweet(&tweet).unwrap();
    }
}
