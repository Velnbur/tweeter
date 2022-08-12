use axum::{extract::Path, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    db,
    records::tweets::Tweet as TweetRecord,
    service::api::{errors::ErrorResponse, schemas::tweets::Tweet as TweetSchema},
};

#[derive(Error, Debug)]
pub enum GetByIdError {
    #[error("tweet not found")]
    TweetNotFound,
    #[error("database error")]
    Database,
}

pub async fn get_by_id(
    Path(id): Path<i64>,
    Extension(pool): Extension<db::Pool>,
) -> Result<impl IntoResponse, GetByIdError> {
    let tweet = TweetRecord::find(id, &pool)
        .await
        .map_err(|err| {
            log::error!("Failed to get tweet by id: {err}");
            GetByIdError::Database
        })?
        .ok_or(GetByIdError::TweetNotFound)?;

    Ok(Json(TweetSchema::from(tweet)))
}

impl IntoResponse for GetByIdError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::TweetNotFound => ErrorResponse::NotFound(self.to_string()),
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
