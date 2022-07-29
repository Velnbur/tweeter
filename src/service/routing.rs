use std::convert::Infallible;
use warp::Filter;

use super::handlers;
use crate::db;

pub fn route(
    db: db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    tasks_route(&db)
}

pub fn tasks_route(
    pool: &db::Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /task/:i64
    let task_by_id = warp::path!("task" / i64)
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::get_task_by_id);

    // * /api/tasks
    let tasks = warp::path("tasks");

    // POST /tasks
    let create = tasks
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handlers::create_task);

    // GET /api/tasks
    let list = tasks
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::get_tasks_list);

    create.or(list).or(task_by_id)
}

fn with_db(pool: db::Pool) -> impl Filter<Extract = (db::Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
