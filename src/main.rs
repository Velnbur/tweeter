mod api;
mod config;
mod db;
mod records;
mod signer;

use crate::config::Config;
use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, TerminalMode};

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

    api::run(config).await;
}
