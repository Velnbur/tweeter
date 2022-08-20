use axum::{Extension, Json};
use tweeter_schemas::{auth_keys::AuthKeysResponse, users::CreateUserRequest};

use crate::{
    records::users::User as UserRecord,
    service::api::{auth, errors::ErrorResponse},
};

pub async fn handler(
    Json(body): Json<CreateUserRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<AuthKeysResponse>, ErrorResponse> {
    let mut user: UserRecord = body.into();

    let (priv_key, pub_key) = auth::generate_keys();

    user.public_key = pub_key;

    let user = user.create(&pool).await.map_err(|err| match err {
        crate::records::errors::Errors::InvalidUsername => ErrorResponse::Conflict(err.to_string()),
        _ => {
            log::error!("Failed to create user: {err}");
            ErrorResponse::InternalError
        }
    })?;

    Ok(Json(AuthKeysResponse::new(user.public_key, priv_key)))
}
