use clap::Parser;

use crate::{config::Config, records, service};

/// Simple backend application
#[derive(Parser, Debug)]
struct Args {
    /// Path to config
    #[clap(short, long, value_parser)]
    pub config: String,

    /// Migrate tables in db
    #[clap(short, long, value_parser)]
    pub migrate: bool,
}

pub async fn run() {
    let args = Args::parse();

    let config = Config::from_file(args.config)
        .await
        .expect("failed to create config");

    if args.migrate {
        records::migrations::migrate(&config.db)
            .await
            .expect("Failed to migrate database");
        log::info!("migrations applied");
    }

    service::run(config).await;
}
