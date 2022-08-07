mod cli;
mod config;
mod db;
mod records;
mod service;

#[tokio::main]
async fn main() {
    cli::run().await;
}
