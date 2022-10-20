use sha3::{Digest, Sha3_256};
use thiserror::Error;
use tweeter_models::tweet::Tweet;
use tweeter_mq::Consumer;
use tweeter_repos::tweets::TweetsRepo;

const NULL_HASH: &'static str = "000000000000000000000000000";

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to do smth with db: {0}")]
    DB(#[from] tweeter_repos::errors::Errors),
}

#[derive(Clone)]
pub struct Hasher<Q>
where
    for<'a> Q: Consumer<Tweet, Error>,
{
    pool: sqlx::PgPool,
    queue: Q,
    last: Option<Tweet>,
}

impl<Q> Hasher<Q>
where
    for<'a> Q: Consumer<Tweet, Error> + Send,
{
    pub fn new(queue: Q, pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            queue,
            last: None,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let hasher = self.clone();

            let res = self
                .queue
                .consume(|tweet| async move {
                    let (last_hash, last_id) = match hasher.last {
                        Some(l) => (l.hash.clone().unwrap(), Some(l.id)),
                        None => (NULL_HASH.to_string(), None),
                    };

                    let tweet = Tweet {
                        previous_id: last_id,
                        ..tweet
                    };

                    let tweet = Self::hash_tweet(tweet, &last_hash);

                    let _res = TweetsRepo::new(&hasher.pool)
                        .where_id(tweet.id)
                        .update(tweet.clone())
                        .await?;

                    Ok(())
                })
                .await;
        }
    }

    fn hash_tweet(tweet: Tweet, last_hash: &String) -> Tweet {
        let mut hasher = Sha3_256::new();

        hasher.update(tweet.id.to_string().as_bytes());
        hasher.update(tweet.text.as_bytes());
        hasher.update(tweet.timestamp.to_string().as_bytes());
        hasher.update(tweet.user_id.as_bytes());
        hasher.update(tweet.signature.as_bytes());

        hasher.update(last_hash.as_bytes());

        Tweet {
            hash: Some(bs58::encode(hasher.finalize()).into_string()),
            ..tweet
        }
    }
}
