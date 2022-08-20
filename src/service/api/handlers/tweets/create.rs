use axum::{Extension, Json};
use tokio::sync::mpsc::Sender;
use tweeter_schemas::tweets::{CreateTweetRequest, TweetResponse};

use crate::{
    records::{
        errors::Errors as RecordErrors, tweets::Tweet as TweetRecord, users::User as UserRecord,
    },
    service::api::{
        auth::{self, craber::Claims},
        errors::ErrorResponse,
    },
};

pub async fn handler(
    claims: Claims,
    Json(body): Json<CreateTweetRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(chan): Extension<Sender<TweetRecord>>,
) -> Result<Json<TweetResponse>, ErrorResponse> {
    let user = UserRecord::find(claims.pub_key, &pool)
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => ErrorResponse::Unauthorized,
            _ => {
                log::error!("Failed to get tweet by id: {err}");
                ErrorResponse::InternalError
            }
        })?;

    let mut tweet: TweetRecord = body.into();

    tweet.user_id = user.public_key;

    auth::verify_tweet(&tweet).map_err(|err| {
        log::info!("Failed to verify signature: {err}");
        ErrorResponse::Forbidden(err.to_string())
    })?;

    let tweet = tweet.create(&pool).await.map_err(|err| {
        log::error!("Failed to insert tweet: {err}");
        ErrorResponse::InternalError
    })?;

    chan.send(tweet.clone()).await.map_err(|err| {
        log::error!("Failed to send tweet: {err}");
        ErrorResponse::InternalError
    })?;

    Ok(Json(TweetResponse::from(tweet)))
}
