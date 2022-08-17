use axum::{extract::Path, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    records::{errors::Errors as RecordErrors, users::User as UserRecord},
    service::api::{errors::ErrorResponse, schemas::users::User as UserSchema},
};

use super::IMAGE_EXPR_SECS;

pub async fn handler(
    Path(pub_key): Path<String>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<impl IntoResponse, Errors> {
    let mut user = UserRecord::find(pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => Errors::UserNotFound,
            _ => {
                log::error!("Failed to get user: {err}");
                Errors::Database
            }
        })?;

    if let Some(image) = user.image_url {
        user.image_url = Some(storage.presign_get(image, IMAGE_EXPR_SECS, None).map_err(
            |err| {
                log::error!("failed to create presigned url for image: {err}");
                Errors::Storage
            },
        )?);
    }

    Ok(Json(UserSchema::from(user)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("user not found")]
    UserNotFound,
    #[error("storage error")]
    Storage,
    #[error("databse error")]
    Database,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::NotFound(self.to_string()),
            Self::Database | Self::Storage => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
