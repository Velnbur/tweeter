use warp::reply::Reply;

use crate::db;
use crate::records;
use super::schemas;
use super::responses::InternalError;

pub async fn create(
    body: schemas::users::User,
    db: db::Pool,
) -> Result<impl Reply, warp::Rejection> {
    let user = match records::users::User::create(body.data.key.id, &db).await {
        Ok(u) => u,
        Err(_) => return Ok(InternalError::new(String::from("failed to get user")).into_response())
    };

    Ok(warp::reply::json(&schemas::users::User::from(user)).into_response())
}
