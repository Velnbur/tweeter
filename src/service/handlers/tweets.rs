use warp::Reply;

use crate::db;
use crate::records::users::User as UserRecord;
use crate::records::tweets::Tweet as TweetRecord;
use crate::service::handlers::responses::{InternalError, NotFound};

use super::schemas::tweets::Tweet as TweetSchema;
use super::schemas::tweets::CreateTweet as CreatTweetSchema;
use super::schemas::tweets::TweetList as TweetListSchema;

pub async fn create(
    pub_key: String,
    req: CreatTweetSchema,
    db: db::Pool,
) -> Result<impl Reply, warp::Rejection> {

    let user = match UserRecord::find(pub_key, &db).await {
        Ok(u) => match u {
            Some(u) => u,
            None => return Ok(NotFound::new(String::from("there is no such user")).into_response()),
        },
        Err(_) => return Ok(InternalError::new(String::from("failed tp get user")).into_response()),
    };

    let tweet = TweetRecord::create(
        req.data.attributes.title,
        req.data.attributes.description,
        user.public_key,
        &db,
    ).await;

    let tweet = match tweet {
        Ok(t) => t,
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&TweetSchema::from(tweet)).into_response())
}

pub async fn get_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweet = match TweetRecord::find(id, &db).await {
        Ok(t) => match t {
            Some(t) => t,
            None => return Ok(NotFound::new(String::from("no such tweet")).into_response()),
        },
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&TweetSchema::from(tweet)).into_response())
}

pub async fn get_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweets = match TweetRecord::select(&db).await {
        Ok(t) => t,
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&TweetListSchema::from(tweets)).into_response())
}
