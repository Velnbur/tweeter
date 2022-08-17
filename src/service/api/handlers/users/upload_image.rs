use std::path::Path;

use axum::{
    extract::{multipart, ContentLengthLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use thiserror::Error;

use crate::{
    records::{errors::Errors as RecordErrors, users::User as UserRecord},
    service::api::{
        auth::craber::Claims, errors::ErrorResponse, schemas::users::User as UserSchema,
    },
};

use super::IMAGE_EXPR_SECS;

const MAX_IMAGE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB

pub async fn handler(
    claims: Claims,
    ContentLengthLimit(mut image): ContentLengthLimit<Multipart, MAX_IMAGE_SIZE>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<impl IntoResponse, Errors> {
    let mut user = UserRecord::find(claims.pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => Errors::UserNotFound,
            _ => {
                log::error!("Failed to find user: {err}");
                Errors::Database
            }
        })?;

    let field = image
        .next_field()
        .await
        .map_err(|err| {
            log::debug!("Failed to get multipart field");
            Errors::MultipartError(err)
        })?
        .ok_or(Errors::NoMutipartField)?;

    let file_name = field.file_name().ok_or(Errors::FileNameError)?.to_string();
    let content_type = field
        .content_type()
        .ok_or(Errors::ContentTypeError)?
        .to_string();
    let content = field.bytes().await.map_err(|err| {
        log::debug!("failed to get form content: {err}");
        Errors::MultipartError(err)
    })?;

    let file_name = create_file_name(&file_name, &user.public_key);

    let data = storage
        .put_object_with_content_type(file_name.clone(), &content, content_type.as_str())
        .await
        .map_err(|err| {
            log::error!("Failed to upload image to storage: {err}");
            Errors::StorageError(err)
        })?;

    if data.status_code() != StatusCode::OK.as_u16() {
        return Err(Errors::StorageBadRequest);
    }

    user.image_url = Some(file_name);

    let mut user = user.update(&pool).await.map_err(|err| {
        log::error!("failed to update user's image: {err}");
        Errors::Database
    })?;

    user.image_url = Some(
        storage
            .presign_get(user.image_url.unwrap(), IMAGE_EXPR_SECS, None)
            .map_err(|err| {
                log::error!("Failed to create presigned url: {err}");
                Errors::StorageError(err)
            })?,
    );

    Ok(Json(UserSchema::from(user)))
}

fn create_file_name(orig: &String, pub_key: &String) -> String {
    let mut res = String::new();

    res.push_str(&pub_key.as_str());
    res.push('-');

    if let Some(ext) = Path::new(orig).extension() {
        if let Some(ext) = ext.to_str() {
            res.push_str(ext);
        }
    }

    res
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("no such user")]
    UserNotFound,
    #[error("failed to get file name")]
    FileNameError,
    #[error("failed to get content type")]
    ContentTypeError,
    #[error("no multipart filed")]
    NoMutipartField,
    #[error("failed to parse multipart form data")]
    MultipartError(#[from] multipart::MultipartError),
    #[error("database error")]
    Database,
    #[error("failed to upload image to storage")]
    StorageError(#[from] s3::error::S3Error),
    #[error("bad request to storage")]
    StorageBadRequest,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::Unauthorized,
            Self::MultipartError(err) => ErrorResponse::BadRequest(err.to_string()),
            Self::NoMutipartField | Self::ContentTypeError | Self::FileNameError => {
                ErrorResponse::BadRequest(self.to_string())
            }
            Self::Database => ErrorResponse::InternalError,
            Self::StorageError(_) => ErrorResponse::InternalError,
            Self::StorageBadRequest => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
