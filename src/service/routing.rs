use std::convert::Infallible;
use warp::Filter;

use crate::db;
use super::handlers;

pub fn route(
    db: db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tweets_route(&db)
        .or(users_routes(&db))
}

fn tweets_route(
    pool: &db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /task/:i64
    let get_by_id = warp::path!("tweet" / i64)
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::tweets::get_by_id);

    // * /api/tasks
    let tweets = warp::path("tweets");

    // POST /tasks
    let create = tweets
        .and(warp::post())
        .and(warp::header::header("Authorization"))
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handlers::tweets::create);

    // GET /api/tasks
    let list = tweets
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::tweets::get_list);

    create.or(list).or(get_by_id)
}

fn users_routes(
    pool: &db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path("auth");

    // POST /auth/register
    let create = root
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
