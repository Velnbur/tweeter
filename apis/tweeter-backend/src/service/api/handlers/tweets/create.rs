use axum::{Extension, Json};
use tokio::sync::mpsc::Sender;
use tweeter_auth::verify_tweet;
use tweeter_models::tweet::Tweet as TweetModel;
use tweeter_schemas::tweets::{CreateTweetRequest, TweetResponse};
use validator::Validate;

use crate::service::api::{auth::Claims, errors::ErrorResponse};

use tweeter_repos::{errors::Errors as RecordErrors, tweets::TweetsRepo, users::UsersRepo};

pub async fn handler(
    claims: Claims,
    Json(body): Json<CreateTweetRequest>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(chan): Extension<Sender<TweetModel>>,
) -> Result<Json<TweetResponse>, ErrorResponse> {
    body.validate()
        .map_err(|err| ErrorResponse::BadRequest(err.to_string()))?;

    let user = UsersRepo::new(&pool)
        .where_pub_key(claims.pub_key)
        .get()
        .await
        .map_err(|err| match err {
            RecordErrors::NotFound => ErrorResponse::Unauthorized,
            _ => {
                log::error!("Failed to get tweet by id: {err}");
                ErrorResponse::InternalError
            }
        })?;

    let mut tweet: TweetModel = body.into();

    tweet.user_id = user.public_key;

    verify_tweet(&tweet).map_err(|err| {
        log::info!("Failed to verify signature: {err}");
        ErrorResponse::Forbidden(err.to_string())
    })?;

    let tweet = TweetsRepo::new(&pool).insert(tweet).await.map_err(|err| {
        log::error!("Failed to insert tweet: {err}");
        ErrorResponse::InternalError
    })?;

    chan.send(tweet.clone()).await.map_err(|err| {
        log::error!("Failed to send tweet: {err}");
        ErrorResponse::InternalError
    })?;

    Ok(Json(TweetResponse::from(tweet)))
}
