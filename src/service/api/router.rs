use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::sync::mpsc::Sender;

use crate::{config::Config, records::tweets::Tweet};

use super::handlers;

pub fn new(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new().merge(auth(cfg)).merge(tweets(cfg, sender))
}

fn auth(cfg: &Config) -> Router {
    Router::new()
        .route(
            "/api/auth/register",
            post(handlers::auth::register::register),
        )
        .layer(Extension(cfg.db.clone()))
}

fn tweets(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new()
        .route("/api/tweets", post(handlers::tweets::create::create))
        .route("/api/tweets/:i64", get(handlers::tweets::by_id::get_by_id))
        .layer(Extension(cfg.db.clone()))
        .layer(Extension(sender.clone()))
}
