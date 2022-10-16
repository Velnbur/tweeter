use clap::Parser;

use crate::{config::Config, migrations, service};

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

    let config = match Config::from_file(args.config).await {
        Ok(cfg) => cfg,
        Err(err) => {
            log::error!("failed to create config: {err}");
            return;
        }
    };

    if args.migrate {
        match migrations::migrate(&config.db).await {
            Ok(_) => (),
            Err(err) => {
                log::error!("failed to migrate: {err}");
                return;
            }
        };
        log::info!("migrations applied");
    }

    service::run(config).await;
}
