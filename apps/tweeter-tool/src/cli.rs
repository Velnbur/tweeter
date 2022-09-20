use clap::{Parser, Subcommand};
use tweeter_auth::generate_keys;

use crate::{create_token::create_token, sign_tweet::sing_tweet};

/// A small tool for testing and tweaking tweeter project
#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and sign tweet
    SignTweet,
    /// Create craber token
    CreateToken,
    /// Generate keys
    GenerateKeys,
}

pub fn run() {
    let args = Args::parse();

    match args.command {
        Commands::SignTweet => sing_tweet(),
        Commands::CreateToken => create_token(),
        Commands::GenerateKeys => {
            let (priv_key, pub_key) = generate_keys();

            println!("Private key: {}", priv_key);
            println!("Public ley: {}", pub_key);
        }
    }
}
