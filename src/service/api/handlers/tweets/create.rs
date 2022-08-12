use axum::{response::IntoResponse, Extension, Json};
use thiserror::Error;
use tokio::sync::mpsc::Sender;

use crate::{
    db::Pool,
    records::{tweets::Tweet as TweetRecord, users::User as UserRecord},
    service::api::{
        auth::{self, craber::Claims},
        errors::ErrorResponse,
        schemas::tweets::{CreateTweet as CreateTweetSchema, Tweet as TweetSchema},
    },
};

pub async fn create(
    claims: Claims,
    Json(body): Json<CreateTweetSchema>,
    Extension(db): Extension<Pool>,
    Extension(chan): Extension<Sender<TweetRecord>>,
) -> Result<impl IntoResponse, CreateError> {
    let user = UserRecord::find(claims.pub_key, &db)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {}", err);
            CreateError::Database
        })?
        .ok_or(CreateError::UserNotFound)?;

    let mut tweet: TweetRecord = body.into();

    tweet.user_id = user.public_key;

    auth::verify_tweet(&tweet).map_err(|err| {
        log::debug!("Failed to verify signature: {err}");
        CreateError::FailedToVerify
    })?;

    let tweet = tweet.create(&db).await.map_err(|err| {
        log::error!("Failed to insert tweet: {err}");
        CreateError::Database
    })?;

    chan.send(tweet.clone()).await.map_err(|err| {
        log::error!("Failed to send tweet: {err}");
        CreateError::FailedToSend
    })?;

    Ok(Json(TweetSchema::from(tweet)))
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("failed to verify tweet")]
    FailedToVerify,
    #[error("user not found")]
    UserNotFound,
    #[error("database error")]
    Database,
    #[error("failed to send tweet to hasher")]
    FailedToSend,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::Unauthorized,
            Self::FailedToVerify => ErrorResponse::Forbidden(self.to_string()),
            _ => ErrorResponse::InternalError,
        };

        resp.into_response()
    }
}