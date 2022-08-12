use axum::{
    extract::{multipart, ContentLengthLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use thiserror::Error;

use crate::{
    db,
    records::users::User as UserRecord,
    service::api::{
        auth::craber::Claims, errors::ErrorResponse, schemas::users::User as UserSchema,
    },
};

const MAX_IMAGE_SIZE: u64 = 1025 * 1025 * 10; // 10 MB

pub async fn handler(
    claims: Claims,
    ContentLengthLimit(mut image): ContentLengthLimit<Multipart, MAX_IMAGE_SIZE>,
    Extension(pool): Extension<db::Pool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<impl IntoResponse, Errors> {
    let mut user = UserRecord::find(claims.pub_key, &pool)
        .await
        .map_err(|err| {
            log::error!("Failed to find user: {err}");
            Errors::Database
        })?
        .ok_or(Errors::UserNotFound)?;

    let field = image
        .next_field()
        .await
        .map_err(|err| {
            log::debug!("Failed to get multipart field");
            Errors::MultipartError(err)
        })?
        .ok_or(Errors::NoMutipartField)?;
    let file_name = field.file_name().ok_or(Errors::FileNameError)?.to_string();
    let content = field.bytes().await.map_err(|err| {
        log::debug!("failed to get form content: {err}");
        Errors::MultipartError(err)
    })?;

    let data = storage
        .put_object(file_name.clone(), &content)
        .await
        .map_err(|err| {
            log::error!("Failed to upload image to storage: {err}");
            Errors::StorageError(err)
        })?;

    if data.status_code() != StatusCode::OK.as_u16() {
        return Err(Errors::StorageBadRequest);
    }

    let mut image_url = String::from(storage.region.endpoint().as_str());

    image_url.push('/');
    image_url.push_str(&storage.name);
    image_url.push('/');
    image_url.push_str(&user.public_key.as_str());
    image_url.push_str(file_name.as_str());

    user.image_url = Some(image_url);

    let user = user.update(&pool).await.map_err(|err| {
        log::error!("failed to update user's image: {err}");
        Errors::Database
    })?;

    Ok(Json(UserSchema::from(user)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("no such user")]
    UserNotFound,
    #[error("failed to get file name")]
    FileNameError,
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
            Self::NoMutipartField => ErrorResponse::BadRequest(self.to_string()),
            Self::FileNameError => ErrorResponse::BadRequest(self.to_string()),
            Self::Database => ErrorResponse::InternalError,
            Self::StorageError(_) => ErrorResponse::InternalError,
            Self::StorageBadRequest => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
