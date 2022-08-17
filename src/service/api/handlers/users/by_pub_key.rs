use axum::{extract::Path, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    records::{errors::Errors as RecordErrors, users::User as UserRecord},
    service::api::{errors::ErrorResponse, schemas::users::User as UserSchema},
};

pub async fn handler(
    Path(pub_key): Path<String>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, Errors> {
    let user = UserRecord::find(pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => Errors::UserNotFound,
            _ => {
                log::error!("Failed to get user: {err}");
                Errors::Database
            }
        })?;

    Ok(Json(UserSchema::from(user)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("user not found")]
    UserNotFound,
    #[error("databse error")]
    Database,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::NotFound(self.to_string()),
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
