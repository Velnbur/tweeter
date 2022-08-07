use openssl::base64;
use tokio::sync::mpsc::Receiver;

use crate::db::Pool;
use crate::records::tweets::Tweet;
use openssl::sha::Sha256;

pub struct Singer {
    db: Pool,
    chan: Receiver<Tweet>,
    hasher: Sha256,
    last: Option<Tweet>,
}

const NULL_HASH: &[u8] = b"000000000000000000000000000";

impl Singer {
    pub fn new(receiver: Receiver<Tweet>, db: Pool) -> Self {
        Self {
            db,
            chan: receiver,
            hasher: Sha256::default(),
            last: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let tweet = self.chan.recv().await.unwrap();

            if let Some(t) = &self.last {
                self.hasher.update(t.hash.as_ref().unwrap().as_bytes())
            } else {
                self.hasher.update(NULL_HASH)
            }

            self.hasher.update(tweet.id.to_string().as_bytes());
            self.hasher.update(tweet.title.as_bytes());
            self.hasher.update(tweet.description.as_bytes());
            self.hasher.update(tweet.signature.as_bytes());

            let hash = self.hasher.finish();

            self.last = Some(
                tweet
                    .update(base64::encode_block(&hash), &self.db)
                    .await
                    .unwrap(),
            );
        }
    }
}
