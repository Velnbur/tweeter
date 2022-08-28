mod auth;
mod errors;
mod handlers;
mod router;

use tokio::sync::mpsc::Sender;
use tweeter_models::tweet::Tweet;

use crate::config;

pub async fn run(cfg: config::Config, sender: Sender<Tweet>) {
    axum::Server::bind(&cfg.server.into())
        .serve(router::new(&cfg, &sender).into_make_service())
        .await
        .expect("service failed to start");
}
