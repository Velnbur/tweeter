use warp::Reply;

use crate::db;
use crate::records;
use crate::service::handlers::responses::{InternalError, NotFound};
use crate::service::handlers::schemas;


pub async fn create(
    pub_key: String,
    req: schemas::tweets::CreateTweet,
    db: db::Pool,
) -> Result<impl Reply, warp::Rejection> {
    let user = match records::users::User::find(pub_key, &db).await {
        Ok(u) => match u {
            Some(u) => u,
            None => return Ok(NotFound::new(String::from("there is no such user")).into_response()),
        },
        Err(_) => return Ok(InternalError::new(String::from("failed tp get user")).into_response()),
    };

    let tweet = records::tweets::Tweet::create(
        req.data.attributes.title,
        req.data.attributes.description,
        user.public_key,
        &db,
    )
    .await;

    let tweet = match tweet {
        Ok(t) => t,
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&schemas::tweets::Tweet::from(tweet)).into_response())
}

pub async fn get_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweet = match records::tweets::Tweet::find(id, &db).await {
        Ok(t) => match t {
            Some(t) => t,
            None => return Ok(NotFound::new(String::from("no such tweet")).into_response()),
        },
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&schemas::tweets::Tweet::from(tweet)).into_response())
}

pub async fn get_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tweets = match records::tweets::Tweet::select(&db).await {
        Ok(t) => t,
        Err(_) => {
            return Ok(InternalError::new(String::from("failed to get tweet")).into_response())
        }
    };

    Ok(warp::reply::json(&schemas::tweets::TweetList::from(tweets)).into_response())
}
