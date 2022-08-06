use std::convert::Infallible;

use super::schemas;
use crate::records;
use thiserror::Error;
use warp::Rejection;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("invalid username")]
    InvalidName,
    #[error("user not found")]
    Unauthorized,
    #[error("tweet not found")]
    TweetNotFound,
    #[error("failed to generate keys: {0}")]
    GenerateKeys(#[from] openssl::error::ErrorStack),
    #[error("database error: {0}")]
    Database(#[from] records::errors::Errors),
}

impl warp::reject::Reject for Errors {}

pub async fn handle_rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    if err.is_not_found() {
        return Ok(schemas::errors::Errors::not_found(
            "invalid path".to_string(),
        ));
    }

    let err = err.find::<Errors>().unwrap();

    match err {
        Errors::TweetNotFound => Ok(schemas::errors::Errors::not_found(
            "tweet not found".to_string(),
        )),
        Errors::Unauthorized => Ok(schemas::errors::Errors::unauthorized()),
        Errors::InvalidName => Ok(schemas::errors::Errors::conflict()),
        Errors::SignValidation(_) => Ok(schemas::errors::Errors::unauthorized()),
        Errors::GenerateKeys(err) => Ok(schemas::errors::Errors::internal_error(Some(
            err.to_string(),
        ))),
        Errors::Database(err) => Ok(schemas::errors::Errors::internal_error(Some(
            err.to_string(),
        ))),
    }
}
