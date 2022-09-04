use axum::{extract::Query, Extension, Json};
use s3::error::S3Error;
use sqlx::PgPool;
use tweeter_models::{tweet::Tweet, user::User};
use tweeter_schemas::{
    include::Include as IncludeTrait,
    query::{Include, Pagination},
    resource_type::ResourceType,
    tweets::TweetListResponse,
};

use crate::{
    records::{
        errors::Errors,
        tweets::TweetsRepo,
        users::{self, UsersRepo},
    },
    service::api::{errors::ErrorResponse, IMAGE_EXPR_SECS},
};

pub async fn handler(
    Query(pagination): Query<Pagination>,
    Query(query): Query<Include>,
    Extension(pool): Extension<sqlx::PgPool>,
    Extension(storage): Extension<s3::Bucket>,
) -> Result<Json<TweetListResponse>, ErrorResponse> {
    let tweets = TweetsRepo::new(&pool)
        .pages(&pagination)
        .select()
        .await
        .map_err(|err| {
            log::error!("Failed to get tweets: {err}");
            ErrorResponse::InternalError
        })?;

    let mut resp = TweetListResponse::from(tweets.clone());

    if let Some(value) = query.include {
        if value == ResourceType::User {
            let users = get_users(&pool, tweets).await.map_err(|err| {
                log::error!("Failed to get corresponding users: {err}");
                ErrorResponse::InternalError
            })?;

            let users = presign_urls(&storage, users).map_err(|err| {
                log::error!("failed to create presigned urls: {err}");
                ErrorResponse::InternalError
            })?;
            resp.include(users);
        }
    }

    Ok(Json(resp))
}

async fn get_users(pool: &PgPool, tweets: Vec<Tweet>) -> Result<Vec<User>, Errors> {
    UsersRepo::new(&pool)
        .where_pub_keys(tweets.into_iter().map(|tweet| tweet.user_id).collect())
        .select()
        .await
}

fn presign_urls(storage: &s3::Bucket, users: Vec<User>) -> Result<Vec<User>, S3Error> {
    users
        .into_iter()
        .map(|user| -> Result<User, S3Error> {
            if let Some(url) = &user.image_url {
                return Ok(User {
                    image_url: Some(storage.presign_get(url, IMAGE_EXPR_SECS, None)?),
                    ..user
                });
            }
            Ok(user)
        })
        .collect()
}
