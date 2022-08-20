use axum::{extract::Path, Extension, Json};
use tweeter_schemas::users::UserResponse;

use crate::{
    records::{errors::Errors as RecordErrors, users::User as UserRecord},
    service::api::errors::ErrorResponse,
};

use super::IMAGE_EXPR_SECS;

pub async fn handler(
    Path(pub_key): Path<String>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<Json<UserResponse>, ErrorResponse> {
    let mut user = UserRecord::find(pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => ErrorResponse::NotFound(err.to_string()),
            _ => {
                log::error!("Failed to get user: {err}");
                ErrorResponse::InternalError
            }
        })?;

    if let Some(image) = user.image_url {
        user.image_url = Some(storage.presign_get(image, IMAGE_EXPR_SECS, None).map_err(
            |err| {
                log::error!("failed to create presigned url for image: {err}");
                ErrorResponse::InternalError
            },
        )?);
    }

    Ok(Json(UserResponse::from(user)))
}
