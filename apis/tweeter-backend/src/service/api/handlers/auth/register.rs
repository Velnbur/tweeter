use axum::{response::IntoResponse, Extension, Json};
use tweeter_auth::generate_keys;
use tweeter_models::user::User as UserModel;
use tweeter_schemas::{auth_keys::AuthKeysResponse, users::CreateUserRequest};
use validator::Validate;

use crate::service::api::errors::ErrorResponse;

use tweeter_repos::{errors::Errors, users::UsersRepo};

pub async fn handler(
    Json(body): Json<CreateUserRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, ErrorResponse> {
    body.validate()
        .map_err(|err| ErrorResponse::BadRequest(err.to_string()))?;

    let mut user: UserModel = body.into();

    let (priv_key, pub_key) = generate_keys();

    user.public_key = pub_key;

    let user = UsersRepo::new(&pool)
        .insert(user)
        .await
        .map_err(|err| match err {
            Errors::InvalidUsername => ErrorResponse::Conflict(err.to_string()),
            _ => {
                log::error!("Failed to create user: {err}");
                ErrorResponse::InternalError
            }
        })?;

    Ok(Json(AuthKeysResponse::new(user.public_key, priv_key)))
}
