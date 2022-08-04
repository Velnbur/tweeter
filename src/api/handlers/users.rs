use warp::reply::Reply;
use warp::Rejection;

use super::rejection::Errors;
use super::schemas::users::CreateUser as CreateUserSchema;
use crate::api::handlers::schemas::auth_keys::AuthKeys;
use crate::api::handlers::utils;
use crate::db;
use crate::records::errors::Errors as RecordError;
use crate::records::users::User as UserRecord;

pub async fn create(body: CreateUserSchema, db: db::Pool) -> Result<impl Reply, Rejection> {
    let mut user: UserRecord = body.into();

    let (private_key, public_key) = utils::generate_keys().map_err(|err| {
        log::error!("Failed to generate keys: {}", err);
        warp::reject::custom(Errors::GenerateKeys(err))
    })?;

    user.public_key = public_key;

    let user = user.create(&db).await.map_err(|err| {
        log::error!("Failed to insert user: {}", err);
        match err {
            RecordError::InvalidUsername => warp::reject::custom(Errors::InvalidName),
            _ => warp::reject::custom(Errors::Database(err)),
        }
    })?;

    Ok(AuthKeys::new(user.public_key, private_key).into_response())
}
