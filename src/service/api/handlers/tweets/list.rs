use axum::{extract::Query, Extension, Json};
use tweeter_schemas::tweets::{Tweet as TweetSchema, TweetListResponse};

use crate::{
    records::{pagination::Pagination, tweets::Tweet as TweetRecord},
    service::api::errors::ErrorResponse,
};

pub async fn handler(
    Query(pagination): Query<Pagination>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<TweetListResponse>, ErrorResponse> {
    let tweets = TweetRecord::select(&pool, &pagination)
        .await
        .map_err(|err| {
            log::error!("Failed to get tweets: {err}");
            ErrorResponse::InternalError
        })?;

    Ok(Json(TweetListResponse {
        data: tweets
            .into_iter()
            .map(|tweet| TweetSchema::from(tweet))
            .collect(),
    }))
}
