use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::sync::mpsc::Sender;

use crate::{config::Config, records::tweets::Tweet};

use super::handlers;

pub fn new(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new()
        .merge(auth(cfg))
        .merge(tweets(cfg, sender))
        .merge(users(cfg))
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
        .route(
            "/api/tweets",
            post(handlers::tweets::create::create).get(handlers::tweets::list::list),
        )
        .route("/api/tweets/:id", get(handlers::tweets::by_id::get_by_id))
        .layer(Extension(cfg.db.clone()))
        .layer(Extension(sender.clone()))
}

fn users(cfg: &Config) -> Router {
    Router::new()
        .route(
            "/api/users/:pub_key",
            get(handlers::users::by_pub_key::by_pub_key),
        )
        .layer(Extension(cfg.db.clone()))
}
