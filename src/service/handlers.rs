use std::convert::Infallible;

use warp::hyper::StatusCode;

use crate::models::Task;

pub async fn get_task(id: i64, db: tokio_postgres::Client) -> Result<impl warp::Reply, Infallible> {
    let task = match Task::find(id, db).await {
        Ok(task) => match task {
            Some(t) => t,
            None => return Ok(StatusCode::NOT_FOUND),
        },
        Err(e) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(warp::reply::json(&task))
}
