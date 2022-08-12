use axum::{response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    db::Pool,
    records::{self, users::User as UserRecord},
    service::api::{
        auth,
        errors::ErrorResponse,
        schemas::{auth_keys::AuthKeys, users::CreateUser as CreateUserSchema},
    },
};

pub async fn register(
    Json(body): Json<CreateUserSchema>,
    Extension(db): Extension<Pool>,
) -> Result<impl IntoResponse, RegisterError> {
    let mut user: UserRecord = body.into();

    let (priv_key, pub_key) = auth::generate_keys();

    user.public_key = pub_key;

    let user = user.create(&db).await.map_err(|err| {
        log::error!("Failed to create user: {err}");
        RegisterError::Database(err)
    })?;

    Ok(Json(AuthKeys::new(user.public_key, priv_key)))
}

#[derive(Error, Debug)]
pub enum RegisterError {
    #[error("database error: {0}")]
    Database(#[from] records::errors::Errors),
}

impl IntoResponse for RegisterError {
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
