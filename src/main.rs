mod service;
mod records;
pub mod db;
mod config;

use clap::Parser;
use log::LevelFilter;
use simplelog::{ColorChoice, TerminalMode};
use crate::config::Config;

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
    ).expect("Failed to init logger");

    records::migrations::migrate(&config.db)
        .await
        .expect("Failed to migrate");

    service::run(config).await;
}
