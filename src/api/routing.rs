use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use warp::Filter;

use super::handlers;
use crate::{db, records::tweets::Tweet};

pub fn route(
    db: db::Pool,
    sender: Sender<Tweet>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    tweets_route(&db, &sender)
        .or(users_routes(&db))
        .recover(handlers::rejection::handle_rejection)
}

fn tweets_route(
    pool: &db::Pool,
    sender: &Sender<Tweet>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path("api").and(warp::path("tweets"));

    // GET /api/tweets/:i64
    let get_by_id = root
        .and(warp::path::param::<i64>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::tweets::get_by_id);

    // POST /api/tweets
    let create = root
        .and(warp::post())
        .and(warp::header::header("Authorization"))
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and(with_sender(sender.clone()))
        .and_then(handlers::tweets::create);

    // GET /api/tweets
    let list = root
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::tweets::get_list);

    create.or(get_by_id).or(list)
}

fn users_routes(
    pool: &db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let auth = warp::path("api").and(warp::path("auth"));

    // POST /auth/register
    let create = auth
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handlers::users::create);

    create
}

fn with_db(pool: db::Pool) -> impl Filter<Extract = (db::Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_sender(
    sender: Sender<Tweet>,
) -> impl Filter<Extract = (Sender<Tweet>,), Error = Infallible> + Clone {
    warp::any().map(move || sender.clone())
}
