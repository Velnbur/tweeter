mod auth;
mod errors;
mod handlers;
mod router;

use tokio::sync::mpsc::Sender;
use tweeter_models::tweet::Tweet;

use crate::config;

pub async fn run(cfg: config::Config, sender: Sender<Tweet>) {
    log::info!("api is starting at {}...", cfg.server);
    axum::Server::bind(&cfg.server.into())
        .serve(router::new(&cfg, &sender).into_make_service())
        .await
        .expect("service failed to start");
}

pub const IMAGE_EXPR_SECS: u32 = 60 * 60 * 24 * 10000; // FIXME: set valid period
