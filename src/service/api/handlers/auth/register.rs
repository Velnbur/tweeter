use axum::{response::IntoResponse, Extension, Json};
use thiserror::Error;
use tweeter_schemas::{auth_keys::AuthKeysResponse, users::CreateUserRequest};

use crate::{
    records::{self, users::User as UserRecord},
    service::api::{auth, errors::ErrorResponse},
};

pub async fn handler(
    Json(body): Json<CreateUserRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, Errors> {
    let mut user: UserRecord = body.into();

    let (priv_key, pub_key) = auth::generate_keys();

    user.public_key = pub_key;

    let user = user.create(&pool).await.map_err(|err| {
        log::error!("Failed to create user: {err}");
        Errors::Database(err)
    })?;

    Ok(Json(AuthKeysResponse::new(user.public_key, priv_key)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("database error: {0}")]
    Database(#[from] records::errors::Errors),
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let err = match self {
            Self::Database(inner) => match inner {
                records::errors::Errors::InvalidUsername => {
                    ErrorResponse::Conflict(inner.to_string())
                }
                _ => ErrorResponse::InternalError,
            },
        };

        err.into_response()
    }
}
