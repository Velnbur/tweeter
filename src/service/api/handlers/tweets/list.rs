use axum::{extract::Query, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    db,
    records::{pagination::Pagination, tweets::Tweet as TweetRecord},
    service::api::{errors::ErrorResponse, schemas::tweets::TweetList as TweetListSchema},
};

pub async fn list(
    Query(pagination): Query<Pagination>,
    Extension(pool): Extension<db::Pool>,
) -> Result<impl IntoResponse, ListError> {
    let tweets = TweetRecord::select(&pool, &pagination)
        .await
        .map_err(|err| {
            log::error!("Failed to get tweets: {err}");
            ListError::Database
        })?;

    Ok(Json(TweetListSchema::from(tweets)))
}

#[derive(Error, Debug)]
pub enum ListError {
    #[error("database error")]
    Database,
}

impl IntoResponse for ListError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
