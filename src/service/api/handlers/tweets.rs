use tokio::sync::mpsc::Sender;
use warp::hyper::StatusCode;
use warp::reply::with_status;
use warp::Reply;

use crate::db;
use crate::records::tweets::Tweet as TweetRecord;
use crate::records::users::User as UserRecord;

use super::rejection::Errors;
use super::schemas::tweets::CreateTweet as CreatTweetSchema;
use super::schemas::tweets::Tweet as TweetSchema;
use super::schemas::tweets::TweetList as TweetListSchema;
use super::utils;

pub async fn create(
    pub_key: String,
    req: CreatTweetSchema,
    db: db::Pool,
    chan: Sender<TweetRecord>,
) -> Result<impl Reply, warp::Rejection> {
    let user = UserRecord::find(pub_key, &db)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {}", err);
            warp::reject::custom(Errors::Database(err))
        })?
        .ok_or(warp::reject::custom(Errors::Unauthorized))?;

    let mut tweet: TweetRecord = req.into();

    utils::verify_tweet(&tweet, &user.public_key).map_err(|err| {
        log::error!("Failed to decrypt signature: {}", err);
        warp::reject::custom(Errors::Unauthorized)
    })?;

    tweet.user_id = user.public_key;

    let tweet = tweet.create(&db).await.map_err(|err| {
        log::error!("Failed to insert tweet: {}", err);
        warp::reject::custom(Errors::Database(err))
    })?;

    chan.send(tweet.clone()).await.map_err(|err| {
        log::error!("Failed to send tweet through chan: {}", err);
        Errors::ChannelSend(err)
    })?;

    Ok(with_status(TweetSchema::from(tweet), StatusCode::CREATED))
}

pub async fn get_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweet = TweetRecord::find(id, &db)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {0}", err);
            Errors::Database(err)
        })?
        .ok_or(Errors::TweetNotFound)?;

    if tweet.hash.is_none() {
        return Ok(with_status(TweetSchema::from(tweet), StatusCode::ACCEPTED));
    }
    Ok(with_status(TweetSchema::from(tweet), StatusCode::OK))
}

pub async fn get_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweets = TweetRecord::select(&db).await.map_err(|err| {
        log::error!("Failed to get list of tweets: {}", err);
        Errors::Database(err)
    })?;

    Ok(TweetListSchema::from(tweets).into_response())
}
