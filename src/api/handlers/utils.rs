use openssl::base64;
use openssl::dsa::Dsa;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Verifier;

use crate::records::tweets::Tweet;

const KEY_SIZE: u32 = 1024;

pub fn generate_keys() -> Result<(String, String), ErrorStack> {
    let keys = Dsa::generate(KEY_SIZE)?;

    let private_key = keys.private_key_to_pem()?;
    let public_key = keys.public_key_to_der()?;

    Ok((
        base64::encode_block(&private_key),
        base64::encode_block(&public_key),
    ))
}

pub fn verify_signature(
    pub_key: &String,
    content: &Tweet,
    signature: &String,
) -> Result<bool, ErrorStack> {
    let pkey = PKey::from_dsa(Dsa::public_key_from_der(
        base64::decode_block(pub_key.as_str())?.as_slice(),
    )?)?;

    let verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;

    verifier.verify(signature.as_bytes())
}
