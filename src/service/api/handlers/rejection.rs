use std::convert::Infallible;

use super::schemas;
use crate::records::{self, tweets::Tweet};
use thiserror::Error;
use warp::Rejection;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("invalid username")]
    InvalidName,
    #[error("user not found")]
    Unauthorized,
    #[error("no access")]
    Forbidden,
    #[error("tweet not found")]
    TweetNotFound,
    #[error("database error: {0}")]
    Database(#[from] records::errors::Errors),
    #[error("failed to send through channel: {0}")]
    ChannelSend(#[from] tokio::sync::mpsc::error::SendError<Tweet>),
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
        Errors::Forbidden => Ok(schemas::errors::Errors::forbidden()),
        Errors::InvalidName => Ok(schemas::errors::Errors::conflict()),
        Errors::ChannelSend(err) => Ok(schemas::errors::Errors::internal_error(Some(
            err.to_string(),
        ))),
        Errors::Database(err) => Ok(schemas::errors::Errors::internal_error(Some(
            err.to_string(),
        ))),
    }
}
