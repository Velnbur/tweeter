pub mod craber;

use ecdsa::signature::Verifier;
use elliptic_curve::rand_core::OsRng;
use k256::ecdsa::{Signature, SigningKey, VerifyingKey};

pub fn generate_keys() -> (String, String) {
    let keys = SigningKey::random(&mut OsRng);

    (
        bs58::encode(keys.to_bytes()).into_string(),
        bs58::encode(keys.verifying_key().to_bytes()).into_string(),
    )
}

use thiserror::Error;
use tweeter_models::tweet::Tweet;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("failed to decode: {0}")]
    DecodeError(#[from] bs58::decode::Error),
    #[error("failed to create verifying key: {0}")]
    VerifyKeyError(ecdsa::Error),
    #[error("failed to verify: {0}")]
    VerifyingError(signature::Error),
}

pub fn verify_signature(msg: &String, sign: &String, pub_key: &String) -> Result<(), VerifyError> {
    let key = VerifyingKey::from_sec1_bytes(
        bs58::decode(pub_key)
            .into_vec()
            .map_err(VerifyError::DecodeError)?
            .as_slice(),
    )
    .map_err(VerifyError::VerifyKeyError)?;

    let sign = Signature::from_der(
        &bs58::decode(sign)
            .into_vec()
            .map_err(VerifyError::DecodeError)?,
    )
    .map_err(VerifyError::VerifyingError)?;

    key.verify(msg.as_bytes(), &sign)
        .map_err(VerifyError::VerifyingError)
}

pub fn verify_tweet(tweet: &Tweet) -> Result<(), VerifyError> {
    let mut msg = String::new();

    msg.push_str(&tweet.text.to_owned());
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
    use tweeter_models::tweet::Tweet;

    use super::{generate_keys, verify_signature, verify_tweet};

    #[test]
    fn test_verify_signature() {
        let (priv_key, pub_key) = generate_keys();

        let sign_key =
            SigningKey::from_bytes(bs58::decode(priv_key).into_vec().unwrap().as_slice()).unwrap();

        let msg = "hello, world";
        let sign: Signature = sign_key.sign(msg.as_bytes());

        verify_signature(
            &msg.to_string(),
            &bs58::encode(sign.to_der()).into_string(),
            &pub_key,
        )
        .unwrap();
    }

    #[test]
    fn test_verify_tweet() {
        let (priv_key, pub_key) = generate_keys();

        let sign_key =
            SigningKey::from_bytes(bs58::decode(priv_key).into_vec().unwrap().as_slice()).unwrap();

        let text = "title";
        let timestamp = 123123;

        let mut msg = String::new();

        msg.push_str(&text.to_owned());
        msg.push('\n');
        msg.push_str(&timestamp.to_string());
        msg.push('\n');
        msg.push_str(&pub_key.to_owned());

        let sign: Signature = sign_key.sign(msg.as_bytes());

        let tweet = Tweet {
            id: 0,
            text: text.to_string(),
            timestamp,
            user_id: pub_key.clone(),
            signature: bs58::encode(sign.to_der()).into_string(),
            hash: None,
            previous_id: None,
        };

        verify_tweet(&tweet).unwrap();
    }
}
