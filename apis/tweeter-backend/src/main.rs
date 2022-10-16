mod cli;
mod config;
mod migrations;
mod service;

#[tokio::main]
async fn main() {
    cli::run().await;
}
