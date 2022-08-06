use tokio::sync::mpsc::Receiver;

use crate::db::Pool;
use crate::records::tweets::Tweet;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub struct Singer {
    db: Pool,
    chan: Receiver<Tweet>,
    hasher: Sha3,
    last: Option<Tweet>,
}

const NULL_HASH: &str = "000000000000000000000000000";

impl Singer {
    pub fn new(receiver: Receiver<Tweet>, db: Pool) -> Self {
        Self {
            db: db,
            chan: receiver,
            hasher: Sha3::keccak256(),
            last: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let tweet = self.chan.recv().await.unwrap();

            if let Some(t) = &self.last {
                self.hasher.input_str(t.hash.as_ref().unwrap().as_str())
            } else {
                self.hasher.input_str(NULL_HASH)
            }

            self.hasher.input(tweet.id.to_string().as_bytes());
            self.hasher.input(tweet.title.as_bytes());
            self.hasher.input(tweet.description.as_bytes());
            self.hasher.input(tweet.signature.as_bytes());

            let hash = self.hasher.result_str();
            self.last = Some(tweet.update(hash, &self.db).await.unwrap());
        }
    }
}
