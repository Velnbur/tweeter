use std::{
    io::stdin,
    time::{SystemTime, UNIX_EPOCH},
};
use tweeter_models::tweet::Tweet;
use tweeter_schemas::tweets::CreateTweet;

#[derive(Debug)]
enum State {
    TweetText,
    TweetPublicKey,
    TweetPrivateKey,
}

#[derive(Debug)]
struct Reader {
    tweet: Tweet,
    priv_key: String,
    state: State,
}

impl Reader {
    pub fn new() -> Self {
        println!("Enter tweet text: ");
        Self {
            tweet: Tweet::default(),
            state: State::TweetText,
            priv_key: String::new(),
        }
    }

    pub fn next(&mut self, line: String) -> bool {
        match self.state {
            State::TweetText => {
                self.tweet.text = line;
                self.state = State::TweetPublicKey;
                println!("Enter your public key: ");
                false
            }
            State::TweetPublicKey => {
                self.tweet.user_id = line;
                self.state = State::TweetPrivateKey;
                println!("Enter your private key: ");
                false
            }
            State::TweetPrivateKey => {
                self.priv_key = line;
                self.state = State::TweetPrivateKey;
                println!("Finished!");
                true
            }
        }
    }

    pub fn finish(self) -> Tweet {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("failed to get current time")
            .as_secs();

        let tweet = Tweet {
            timestamp: (timestamp as i32),
            ..self.tweet
        };

        tweeter_auth::sign_tweet(tweet, &self.priv_key).expect("failed to sign tweet")
    }
}

pub fn sing_tweet() {
    let mut reader = Reader::new();
    let mut input = String::new();

    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        input.pop(); // remove '\n' from the end of the line
        if reader.next(input.clone()) {
            break;
        }

        input.clear();
    }

    let tweet = reader.finish();

    let schema = CreateTweet::new(tweet.text, tweet.timestamp, tweet.signature);

    println!(
        "Result request: {}",
        serde_json::to_string(&schema).expect("failed to marshal result")
    );
}
