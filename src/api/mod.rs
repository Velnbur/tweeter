use crate::config;

mod routing;
mod handlers;

pub async fn run(cfg: config::Config) {
    warp::serve(routing::route(cfg.db))
        .run(cfg.server)
        .await;
}