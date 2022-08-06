mod api;
mod config;
mod db;
mod records;
mod signer;

use crate::config::Config;
use signer::Singer;

use clap::Parser;
use log::LevelFilter;
use records::tweets::Tweet;
use simplelog::{ColorChoice, TerminalMode};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

/// Simple backend application
#[derive(Parser, Debug)]
struct Args {
    /// Path to config
    #[clap(short, long, value_parser)]
    pub config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = Config::from_file(args.config);

    simplelog::TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("Failed to init logger");

    records::migrations::migrate(&config.db)
        .await
        .expect("Failed to migrate");

    let (sender, receiver): (Sender<Tweet>, Receiver<Tweet>) = mpsc::channel(1000);

    let mut signer = Singer::new(receiver, config.db.clone());
    tokio::spawn(async move {
        signer.start().await;
    });

    api::run(config, sender).await;
}
