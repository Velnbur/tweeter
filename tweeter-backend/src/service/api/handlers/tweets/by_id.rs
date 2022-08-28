use axum::{extract::Path, Extension, Json};
use tweeter_schemas::tweets::TweetResponse;

use crate::{
    records::{errors::Errors as RecordErrors, tweets::TweetsRepo},
    service::api::errors::ErrorResponse,
};

pub async fn handler(
    Path(id): Path<i64>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<TweetResponse>, ErrorResponse> {
    let tweet = TweetsRepo::new(&pool)
        .where_id(id)
        .get()
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => ErrorResponse::NotFound(err.to_string()),
            _ => {
                log::error!("Failed to get tweet by id: {err}");
                ErrorResponse::InternalError
            }
        })?;

    Ok(Json(TweetResponse::from(tweet)))
}
