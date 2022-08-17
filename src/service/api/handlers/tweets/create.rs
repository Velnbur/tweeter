use axum::{response::IntoResponse, Extension, Json};
use thiserror::Error;
use tokio::sync::mpsc::Sender;

use crate::{
    records::{
        errors::Errors as RecordErrors, tweets::Tweet as TweetRecord, users::User as UserRecord,
    },
    service::api::{
        auth::{self, craber::Claims},
        errors::ErrorResponse,
        schemas::tweets::{CreateTweet as CreateTweetSchema, Tweet as TweetSchema},
    },
};

pub async fn handler(
    claims: Claims,
    Json(body): Json<CreateTweetSchema>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(chan): Extension<Sender<TweetRecord>>,
) -> Result<impl IntoResponse, Errors> {
    let user = UserRecord::find(claims.pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => Errors::UserNotFound,
            _ => {
                log::error!("Failed to get tweet by id: {err}");
                Errors::Database
            }
        })?;

    let mut tweet: TweetRecord = body.into();

    tweet.user_id = user.public_key;

    auth::verify_tweet(&tweet).map_err(|err| {
        log::debug!("Failed to verify signature: {err}");
        Errors::FailedToVerify
    })?;

    let tweet = tweet.create(&pool).await.map_err(|err| {
        log::error!("Failed to insert tweet: {err}");
        Errors::Database
    })?;

    chan.send(tweet.clone()).await.map_err(|err| {
        log::error!("Failed to send tweet: {err}");
        Errors::FailedToSend
    })?;

    Ok(Json(TweetSchema::from(tweet)))
}

#[derive(Error, Debug)]
pub enum Errors {
    #[error("failed to verify tweet")]
    FailedToVerify,
    #[error("user not found")]
    UserNotFound,
    #[error("database error")]
    Database,
    #[error("failed to send tweet to hasher")]
    FailedToSend,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::Unauthorized,
            Self::FailedToVerify => ErrorResponse::Forbidden(self.to_string()),
            _ => ErrorResponse::InternalError,
        };

        resp.into_response()
    }
}
