use axum::{routing::post, Extension, Router};
use tokio::sync::mpsc::Sender;

use crate::{config::Config, records::tweets::Tweet};

use super::handlers;

pub fn new(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new().merge(auth(cfg)).merge(tweets(cfg, sender))
}

fn auth(cfg: &Config) -> Router {
    Router::new()
        .route("/auth/register", post(handlers::auth::register::register))
        .layer(Extension(cfg.db.clone()))
}

fn tweets(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new()
        .route("/tweets", post(handlers::tweets::create::create))
        .layer(Extension(cfg.db.clone()))
        .layer(Extension(sender.clone()))
}
