use warp::Reply;

use crate::db;
use crate::records::users::User as UserRecord;
use crate::records::tweets::Tweet as TweetRecord;
use crate::service::handlers::responses::{internal_error, not_found, json};

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
            None => return Ok(internal_error(String::from("there is no such user"))),
        },
        Err(_) => return Ok(internal_error(String::from("failed tp get user"))),
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
            return Ok(internal_error(String::from("failed to get tweet")))
        }
    };

    Ok(json(&TweetSchema::from(tweet)))
}

pub async fn get_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweet = match TweetRecord::find(id, &db).await {
        Ok(t) => match t {
            Some(t) => t,
            None => return Ok(not_found(String::from("no such tweet"))),
        },
        Err(_) => {
            return Ok(not_found(String::from("failed to get tweet")))
        }
    };

    Ok(json(&TweetSchema::from(tweet)))
}

pub async fn get_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweets = match TweetRecord::select(&db).await {
        Ok(t) => t,
        Err(_) => {
            return Ok(internal_error(String::from("failed to get tweet")))
        }
    };

    Ok(json(&TweetListSchema::from(tweets)))
}
