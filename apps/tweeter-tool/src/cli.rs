use clap::{Parser, Subcommand};
use tweeter_auth::{generate_keys, token::create_token_now};

use crate::{create_token::create_token, sign_tweet::sing_tweet};

/// A small tool for testing and tweaking tweeter project
#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long)]
    stdin: bool,

    #[clap(long)]
    private_key: Option<String>,

    #[clap(long)]
    public_key: Option<String>,
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

    if args.stdin {
        match args.command {
            Commands::SignTweet => sing_tweet(),
            Commands::CreateToken => create_token(),
            Commands::GenerateKeys => {
                let (priv_key, pub_key) = generate_keys();

                println!("Private key: {}", priv_key);
                println!("Public ley: {}", pub_key);
            }
        };
        return;
    }

    match args.command {
        Commands::SignTweet => {
            // TODO:
            panic!("not impelemented");
        }
        Commands::CreateToken => {
            if args.private_key.is_none() || args.public_key.is_none() {
                // TODO:
                panic!("keys are required");
            }
            let pub_key = args.public_key.unwrap();
            let priv_key = args.private_key.unwrap();

            // TODO:
            let token = create_token_now(&pub_key, &priv_key).expect("failed to create token");
            println!("{token}");
        }
        Commands::GenerateKeys => {
            let (priv_key, pub_key) = generate_keys();

            println!("{priv_key}");
            println!("{pub_key}");
        }
    }
}
