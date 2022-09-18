use ecdsa::signature::Signer;
use ecdsa::signature::Verifier;
use elliptic_curve::rand_core::OsRng;
use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use k256::Secp256k1;
use thiserror::Error;
use tweeter_models::tweet::Tweet;

pub fn generate_keys() -> (String, String) {
    let keys = SigningKey::random(&mut OsRng);

    (
        bs58::encode(keys.to_bytes()).into_string(),
        bs58::encode(keys.verifying_key().to_bytes()).into_string(),
    )
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to decode: {0}")]
    DecodeError(#[from] bs58::decode::Error),
    #[error("failed to verify: {0}")]
    SignatureError(#[from] signature::Error),
}

pub fn verify_signature(msg: &String, sign: &String, pub_key: &String) -> Result<(), Error> {
    let decoded_bytes = bs58::decode(pub_key).into_vec()?;

    let key = VerifyingKey::from_sec1_bytes(&decoded_bytes.as_slice())?;

    let sign = Signature::from_der(&bs58::decode(sign).into_vec()?)?;

    key.verify(msg.as_bytes(), &sign)
        .map_err(Error::SignatureError)
}

pub fn sign_msg(msg: &String, priv_key: &String) -> Result<String, Error> {
    let decoded_bytes = bs58::decode(priv_key).into_vec()?;

    let key = SigningKey::from_bytes(&decoded_bytes)?;

    let signature: ecdsa::Signature<Secp256k1> = key.try_sign(msg.as_bytes())?;

    let encoded_signature = bs58::encode(signature.to_der());

    Ok(encoded_signature.into_string())
}

/// create a buffer of chars from tweet that is ready for signing or verifying
fn tweet_to_msg(tweet: &Tweet) -> String {
    let mut msg = String::new();

    msg.push_str(&tweet.text.to_owned());
    msg.push('\n');
    msg.push_str(&tweet.timestamp.to_string());
    msg.push('\n');
    msg.push_str(&tweet.user_id.to_owned());

    msg
}

pub fn verify_tweet(tweet: &Tweet) -> Result<(), Error> {
    let msg = tweet_to_msg(tweet);

    verify_signature(&msg, &tweet.signature, &tweet.user_id)
}

pub fn sign_tweet(tweet: Tweet, key: &String) -> Result<Tweet, Error> {
    let msg = tweet_to_msg(&tweet);

    let signature = sign_msg(&msg, key)?;

    Ok(Tweet { signature, ..tweet })
}

#[cfg(test)]
mod test {

    use tweeter_models::tweet::Tweet;

    use crate::{sign_msg, sign_tweet};

    use super::{generate_keys, verify_signature, verify_tweet};

    #[test]
    fn test_sign_msg() {
        let (priv_key, _) = generate_keys();

        let msg = "hello, world!".to_string();

        sign_msg(&msg, &priv_key).expect("failed to sign message");
    }

    #[test]
    fn test_sign_tweet() {
        let (priv_key, pub_key) = generate_keys();
        let tweet = Tweet {
            id: 0,
            text: "text".to_string(),
            timestamp: 123123,
            user_id: pub_key,
            signature: String::new(),
            hash: None,
            previous_id: None,
        };

        let tweet = sign_tweet(tweet, &priv_key).expect("failed to sign tweet");

        assert!(tweet.signature.len() != 0);
    }

    #[test]
    fn test_verify_signature() {
        let (priv_key, pub_key) = generate_keys();

        let msg = "hello, world".to_string();

        let signature = sign_msg(&msg, &priv_key).expect("failed to sign message");

        verify_signature(&msg, &signature, &pub_key).expect("failed to verify signature");
    }

    #[test]
    fn test_verify_tweet() {
        let (priv_key, pub_key) = generate_keys();

        let tweet = Tweet {
            id: 0,
            text: "test".to_string(),
            timestamp: 123123,
            user_id: pub_key.clone(),
            signature: String::new(), // will be filled later
            hash: None,
            previous_id: None,
        };

        let tweet = sign_tweet(tweet, &priv_key).expect("failed to sign tweet");

        verify_tweet(&tweet).expect("failed to verify tweet");
    }
}
