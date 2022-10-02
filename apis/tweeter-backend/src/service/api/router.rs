use axum::{
    routing::{get, post},
    Extension, Router,
};
use tokio::sync::mpsc::Sender;
use tweeter_models::tweet::Tweet;

use crate::config::Config;

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
            post(handlers::auth::register::handler),
        )
        .route(
            "/api/auth/register/key",
            post(handlers::auth::register_keys::handler),
        )
        .layer(Extension(cfg.db.clone()))
}

fn tweets(cfg: &Config, sender: &Sender<Tweet>) -> Router {
    Router::new()
        .route(
            "/api/tweets",
            post(handlers::tweets::create::handler).get(handlers::tweets::list::handler),
        )
        .route("/api/tweets/:id", get(handlers::tweets::by_id::handler))
        .layer(Extension(cfg.storage.clone()))
        .layer(Extension(cfg.db.clone()))
        .layer(Extension(sender.clone()))
}

fn users(cfg: &Config) -> Router {
    Router::new()
        .route(
            "/api/users/:pub_key",
            get(handlers::users::by_pub_key::handler),
        )
        .route(
            "/api/users/image/upload",
            post(handlers::users::upload_image::handler),
        )
        .layer(Extension(cfg.db.clone()))
        .layer(Extension(cfg.storage.clone()))
}
