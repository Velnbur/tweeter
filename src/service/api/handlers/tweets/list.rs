use axum::{extract::Query, response::IntoResponse, Extension, Json};
use thiserror::Error;
use tweeter_schemas::tweets::{Tweet as TweetSchema, TweetListResponse};

use crate::{
    records::{pagination::Pagination, tweets::Tweet as TweetRecord},
    service::api::errors::ErrorResponse,
};

pub async fn handler(
    Query(pagination): Query<Pagination>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, Errors> {
    let tweets = TweetRecord::select(&pool, &pagination)
        .await
        .map_err(|err| {
            log::error!("Failed to get tweets: {err}");
            Errors::Database
        })?;

    Ok(Json(TweetListResponse {
        data: tweets
            .into_iter()
            .map(|tweet| TweetSchema::from(tweet))
            .collect(),
    }))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("database error")]
    Database,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
