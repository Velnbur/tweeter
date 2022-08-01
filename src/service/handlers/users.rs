use warp::reply::Reply;

use super::responses::InternalError;
use super::schemas::users::User as UserSchema;
use crate::db;
use crate::records::users::User as UserRecord;

pub async fn create(body: UserSchema, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let user: UserRecord = body.into();

    let user = match user.create(&db).await {
        Ok(u) => u,
        Err(_) => return Ok(InternalError::new(String::from("failed to get user")).into_response()),
    };

    Ok(warp::reply::json(&UserSchema::from(user)).into_response())
}
