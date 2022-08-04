use warp::Reply;

use crate::db;
use crate::records::tweets::Tweet as TweetRecord;
use crate::records::users::User as UserRecord;

use super::rejection::Errors;
use super::schemas::tweets::CreateTweet as CreatTweetSchema;
use super::schemas::tweets::Tweet as TweetSchema;
use super::schemas::tweets::TweetList as TweetListSchema;

pub async fn create(
    pub_key: String,
    req: CreatTweetSchema,
    db: db::Pool,
) -> Result<impl Reply, warp::Rejection> {
    let user = UserRecord::find(pub_key, &db)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {}", err);
            warp::reject::custom(Errors::Database(err))
        })?
        .ok_or(warp::reject::custom(Errors::Unauthorized))?;

    let mut tweet: TweetRecord = req.into();
    tweet.user_id = user.public_key;

    let tweet = tweet.create(&db).await.map_err(|err| {
        log::error!("Failed to insert tweet: {}", err);
        Errors::Database(err)
    })?;

    Ok(TweetSchema::from(tweet).into_response())
}

pub async fn get_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweet = TweetRecord::find(id, &db)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {0}", err);
            Errors::Database(err)
        })?
        .ok_or(Errors::TweetNotFound)?;

    Ok(TweetSchema::from(tweet).into_response())
}

pub async fn get_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweets = TweetRecord::select(&db).await.map_err(|err| {
        log::error!("Failed to get list of tweets: {}", err);
        Errors::Database(err)
    })?;

    Ok(TweetListSchema::from(tweets).into_response())
}
