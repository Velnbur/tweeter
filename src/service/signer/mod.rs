use sha3::{Digest, Sha3_256};
use tokio::sync::mpsc::Receiver;

use crate::db::Pool;
use crate::records::tweets::Tweet;

pub struct Hasher {
    db: Pool,
    chan: Receiver<Tweet>,
    last: Option<Tweet>,
}

const NULL_HASH: &'static str = "000000000000000000000000000";

impl Hasher {
    pub fn new(receiver: Receiver<Tweet>, db: Pool) -> Self {
        Self {
            db,
            chan: receiver,
            last: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let mut tweet = self.chan.recv().await.unwrap();
            let last_hash = match self.last.clone() {
                Some(l) => l.hash.unwrap(),
                None => NULL_HASH.to_string(),
            };

            Self::hash_tweet(&mut tweet, &last_hash);

            self.last = match tweet.update(&self.db).await {
                Err(err) => {
                    log::error!("Failed to hash tweet with error: {}", err);
                    continue;
                }
                Ok(tweet) => Some(tweet),
            };
        }
    }

    fn hash_tweet(tweet: &mut Tweet, last_hash: &String) {
        let mut hasher = Sha3_256::new();

        hasher.update(tweet.id.to_string().as_bytes());
        hasher.update(tweet.title.as_bytes());
        hasher.update(tweet.description.as_bytes());
        hasher.update(tweet.timestamp.to_string().as_bytes());
        hasher.update(tweet.user_id.as_bytes());
        hasher.update(tweet.signature.as_bytes());

        hasher.update(last_hash.as_bytes());

        tweet.hash = Some(base64::encode(hasher.finalize()));
    }
}
