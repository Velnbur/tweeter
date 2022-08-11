mod auth;
mod handlers;
mod router;
mod schemas;

use tokio::sync::mpsc::Sender;

use crate::{config, records::tweets::Tweet};

pub async fn run(cfg: config::Config, sender: Sender<Tweet>) {
    axum::Server::bind(&cfg.server.into())
        .serve(router::new(&cfg, &sender).into_make_service())
        .await
        .expect("service failed to start");
}
