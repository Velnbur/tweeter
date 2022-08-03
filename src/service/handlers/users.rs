use serde::de::Unexpected::Str;
use warp::reply::Reply;

use super::schemas::users::CreateUser as CreateUserSchema;
use crate::db;
use crate::records::users::{User as UserRecord, UsersError};
use crate::service::handlers::responses::{conflict, internal_error, json};
use crate::service::handlers::schemas::auth_keys::AuthKeys;
use crate::service::handlers::utils;

pub async fn create(body: CreateUserSchema, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let mut user: UserRecord = body.into();

    let (private_key, public_key) = match utils::generate_keys() {
        Ok(keys) => keys,
        Err(err) => {
            log::error!("Failed to generate keys: {}", err);
            return Ok(internal_error(String::from("failed to generate keys")));
        }
    };

    user.public_key = public_key;

    let user = match user.create(&db).await {
        Ok(u) => u,
        Err(err) => {
            log::debug!("Failed to insert user: {}", err);
            return match err {
                UsersError::InvalidUsername => Ok(conflict()),
                _ => Ok(internal_error(String::from("failed to create user")))
            }
        }
    };

    Ok(json(&AuthKeys::new(user.public_key, private_key)))
}
