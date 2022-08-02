use warp::reply::Reply;

use super::schemas::users::User as UserSchema;
use crate::db;
use crate::records::users::User as UserRecord;
use crate::service::handlers::responses::{internal_error, json};

pub async fn create(body: UserSchema, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let user: UserRecord = body.into();

    let user = match user.create(&db).await {
        Ok(u) => u,
        Err(_) => return Ok(internal_error(String::from("failed to get user"))),
    };

    Ok(json(&UserSchema::from(user)))
}
