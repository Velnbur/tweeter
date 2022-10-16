use std::path::Path;

use axum::{
    extract::{ContentLengthLimit, Multipart},
    http::StatusCode,
    Extension, Json,
};
use tweeter_repos::{errors::Errors as RecordErrors, users::UsersRepo};
use tweeter_schemas::users::UserResponse;

use crate::service::api::{auth::Claims, errors::ErrorResponse, IMAGE_EXPR_SECS};

const MAX_IMAGE_SIZE: u64 = 1024 * 1024 * 10; // 10 MB

const NO_IMAGE_FIELD: &str = "no 'image' field";
const NO_FILE_NAME: &str = "failed to get filename";
const NO_MEDIA_TYPE: &str = "failed to get media type";

pub async fn handler(
    claims: Claims,
    ContentLengthLimit(mut image): ContentLengthLimit<Multipart, MAX_IMAGE_SIZE>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<Json<UserResponse>, ErrorResponse> {
    let mut user = UsersRepo::new(&pool)
        .where_pub_key(claims.pub_key)
        .get()
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => ErrorResponse::Unauthorized,
            _ => {
                log::error!("Failed to find user: {err}");
                ErrorResponse::InternalError
            }
        })?;

    let field = image
        .next_field()
        .await
        .map_err(|err| {
            log::info!("Failed to get multipart field");
            ErrorResponse::BadRequest(err.to_string())
        })?
        .ok_or(ErrorResponse::BadRequest(NO_IMAGE_FIELD.to_string()))?;

    let file_name = field
        .file_name()
        .ok_or(ErrorResponse::BadRequest(NO_FILE_NAME.to_string()))?
        .to_string();
    let content_type = field
        .content_type()
        .ok_or(ErrorResponse::BadRequest(NO_MEDIA_TYPE.to_string()))?
        .to_string();
    let content = field.bytes().await.map_err(|err| {
        log::info!("failed to get form content: {err}");
        ErrorResponse::InternalError
    })?;

    let file_name = create_file_name(&file_name, &user.public_key);

    let data = storage
        .put_object_with_content_type(file_name.clone(), &content, content_type.as_str())
        .await
        .map_err(|err| {
            log::error!("Failed to upload image to storage: {err}");
            ErrorResponse::InternalError
        })?;

    if data.status_code() != StatusCode::OK.as_u16() {
        log::info!("request to s3 bucket failed");
        return Err(ErrorResponse::InternalError);
    }

    user.image_url = Some(file_name);

    let mut user = UsersRepo::new(&pool).update(user).await.map_err(|err| {
        log::error!("failed to update user's image: {err}");
        ErrorResponse::InternalError
    })?;

    user.image_url = Some(
        storage
            .presign_get(user.image_url.unwrap(), IMAGE_EXPR_SECS, None)
            .map_err(|err| {
                log::error!("Failed to create presigned url: {err}");
                ErrorResponse::InternalError
            })?,
    );

    Ok(Json(UserResponse::from(user)))
}

fn create_file_name(orig: &String, pub_key: &String) -> String {
    let mut res = String::new();

    res.push_str(&pub_key.as_str());
    res.push('-');

    if let Some(ext) = Path::new(orig).extension() {
        if let Some(ext) = ext.to_str() {
            res.push('.');
            res.push_str(ext);
        }
    }

    res
}
