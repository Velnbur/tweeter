use axum::{response::IntoResponse, Extension, Json};
use tweeter_models::user::User as UserModel;
use tweeter_schemas::users::{CreateUserRequest, UserResponse};
use validator::Validate;

use crate::service::api::errors::ErrorResponse;

use tweeter_repos::{errors::Errors, users::UsersRepo};

pub async fn handler(
    Json(body): Json<CreateUserRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, ErrorResponse> {
    body.validate()
        .map_err(|err| ErrorResponse::BadRequest(err.to_string()))?;

    let user: UserModel = body.into();

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

    Ok(Json(UserResponse::from(user)))
}
