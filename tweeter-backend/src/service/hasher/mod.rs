use std::process::exit;

use crate::records::tweets::Tweet;
use sha3::{Digest, Sha3_256};
use sqlx::PgPool;
use tokio::sync::mpsc::Receiver;

pub struct Hasher {
    pool: PgPool,
    chan: Receiver<Tweet>,
    last: Option<Tweet>,
}

const NULL_HASH: &'static str = "000000000000000000000000000";

impl Hasher {
    pub fn new(receiver: Receiver<Tweet>, pool: PgPool) -> Self {
        Self {
            pool,
            chan: receiver,
            last: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let mut tweet = match self.chan.recv().await {
                Some(val) => val,
                None => {
                    log::error!("channel for hashing is not acceptable");
                    exit(1);
                }
            };
            let (last_hash, last_id) = match self.last.clone() {
                Some(l) => {
                    tweet.previous_id = Some(l.id);
                    (l.hash.unwrap(), Some(l.id))
                }
                None => (NULL_HASH.to_string(), None),
            };
            tweet.previous_id = last_id;

            Self::hash_tweet(&mut tweet, &last_hash);

            self.last = match tweet.update(&self.pool).await {
                Err(err) => {
                    log::error!("Failed to hash tweet with error: {err}");
                    continue;
                }
                Ok(tweet) => {
                    log::info!("successfuly hashed: {}", tweet.id);
                    Some(tweet)
                }
            };
        }
    }

    fn hash_tweet(tweet: &mut Tweet, last_hash: &String) {
        let mut hasher = Sha3_256::new();

        hasher.update(tweet.id.to_string().as_bytes());
        hasher.update(tweet.text.as_bytes());
        hasher.update(tweet.timestamp.to_string().as_bytes());
        hasher.update(tweet.user_id.as_bytes());
        hasher.update(tweet.signature.as_bytes());

        hasher.update(last_hash.as_bytes());

        tweet.hash = Some(bs58::encode(hasher.finalize()).into_string());
    }
}
