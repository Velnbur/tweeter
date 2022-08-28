use axum::{extract::Query, Extension, Json};
use tweeter_schemas::{
    query::{Include, Pagination},
    resource_type::ResourceType,
    tweets::TweetListResponse,
    users::User,
};

use crate::{
    records::{tweets::TweetsRepo, users::UsersRepo},
    service::api::errors::ErrorResponse,
};

pub async fn handler(
    Query(pagination): Query<Pagination>,
    Query(include_user): Query<Include>,
    Extension(pool): Extension<sqlx::PgPool>,
) -> Result<Json<TweetListResponse>, ErrorResponse> {
    let tweets = TweetsRepo::new(&pool)
        .pages(&pagination)
        .select()
        .await
        .map_err(|err| {
            log::error!("Failed to get tweets: {err}");
            ErrorResponse::InternalError
        })?;

    let mut resp = TweetListResponse::from(tweets.clone()); // TODO:

    if include_user.include == ResourceType::User {
        let users = UsersRepo::new(&pool)
            .where_pub_keys(tweets.into_iter().map(|tweet| tweet.user_id).collect())
            .select()
            .await
            .map_err(|err| {
                log::error!("Failed to get corresponding users: {err}");
                ErrorResponse::InternalError
            })?;

        resp.include = Some(users.into_iter().map(|user| User::from(user)).collect());
    }

    Ok(Json(resp))
}
