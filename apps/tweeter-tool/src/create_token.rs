use std::{
    io::stdin,
    time::{SystemTime, UNIX_EPOCH},
};

use tweeter_auth::sign_msg;

enum State {
    PrivateKey,
    PublicKey,
}

pub struct Reader {
    state: State,
    priv_key: String,
    pub_key: String,
}

impl Reader {
    pub fn new() -> Self {
        println!("Enter private key: ");
        Self {
            state: State::PrivateKey,
            priv_key: String::default(),
            pub_key: String::default(),
        }
    }

    pub fn next(&mut self, line: String) -> bool {
        match self.state {
            State::PrivateKey => {
                self.priv_key = line;
                self.state = State::PublicKey;
                println!("Enter public key: ");
                false
            }
            State::PublicKey => {
                self.pub_key = line;
                println!("Finish!");
                true
            }
        }
    }

    pub fn finish(self) -> (String, String) {
        (self.priv_key, self.pub_key)
    }
}

const DELIMITER_SYMBOL: char = '.';

pub fn create_token() {
    let mut reader = Reader::new();
    let mut input = String::new();

    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        input.pop();
        if reader.next(input.clone()) {
            break;
        }
        input.clear();
    }

    let (priv_key, pub_key) = reader.finish();

    let mut msg = String::new();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("failed to get current time")
        .as_secs();

    msg.push_str(&timestamp.to_string());
    msg.push(DELIMITER_SYMBOL);
    msg.push_str(pub_key.as_str());

    let signature = sign_msg(&msg, &priv_key).expect("failed to sign message");

    msg.push(DELIMITER_SYMBOL);
    msg.push_str(signature.as_str());

    println!("Craber token: {}", msg);
}
