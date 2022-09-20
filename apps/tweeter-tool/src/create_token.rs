use std::io::stdin;

use tweeter_auth::token::create_token_now;

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

pub fn create_token() {
    let mut reader = Reader::new();
    let mut input = String::new();

    loop {
        stdin()
            .read_line(&mut input)
            // TODO:
            .expect("Did not enter a correct string");

        input.pop();
        if reader.next(input.clone()) {
            break;
        }
        input.clear();
    }

    let (priv_key, pub_key) = reader.finish();

    // TODO:
    let token = create_token_now(&pub_key, &priv_key).expect("failed to create token");

    println!("Craber token: {}", token);
}
