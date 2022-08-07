use tokio::sync::mpsc::Sender;

use crate::{config, records::tweets::Tweet};

mod handlers;
mod routing;

pub async fn run(cfg: &config::Config, sender: Sender<Tweet>) {
    warp::serve(routing::route(cfg.db, sender))
        .run(cfg.server)
        .await;
}
