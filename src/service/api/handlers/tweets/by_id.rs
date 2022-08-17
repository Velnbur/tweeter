use axum::{extract::Path, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    records::{errors::Errors as RecordErrors, tweets::Tweet as TweetRecord},
    service::api::{errors::ErrorResponse, schemas::tweets::Tweet as TweetSchema},
};

pub async fn handler(
    Path(id): Path<i64>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<impl IntoResponse, Errors> {
    let tweet = TweetRecord::find(id, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => Errors::TweetNotFound,
            _ => {
                log::error!("Failed to get tweet by id: {err}");
                Errors::Database
            }
        })?;

    Ok(Json(TweetSchema::from(tweet)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("tweet not found")]
    TweetNotFound,
    #[error("database error")]
    Database,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::TweetNotFound => ErrorResponse::NotFound(self.to_string()),
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
