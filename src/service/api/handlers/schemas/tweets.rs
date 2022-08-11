use serde::{Deserialize, Serialize};
use warp::hyper::header::CONTENT_TYPE;
use warp::hyper::{http, Body, StatusCode};
use warp::Reply;

use super::key::Key;
use super::relation::Relation;
use super::resource_type::ResourceType;
use super::JSON_CONTENT_TYPE;
use crate::records;

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetAttributes {
    pub title: String,
    pub description: String,
    pub signature: String,
    pub timestamp: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRelations {
    pub author: Relation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetData {
    #[serde(flatten)]
    pub key: Key,

    pub attributes: TweetAttributes,
    pub relationships: TweetRelations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub data: TweetData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetList {
    data: Vec<TweetData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetAttributes {
    pub title: String,
    pub description: String,
    pub timestamp: i32,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetData {
    #[serde(rename = "type")]
    pub _type: ResourceType,
    pub attributes: CreateTweetAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweet {
    pub data: CreateTweetData,
}

impl Into<records::tweets::Tweet> for CreateTweet {
    fn into(self) -> records::tweets::Tweet {
        records::tweets::Tweet {
            id: 0,
            title: self.data.attributes.title,
            description: self.data.attributes.description,
            signature: self.data.attributes.signature,
            timestamp: self.data.attributes.timestamp,
            hash: None,
            user_id: String::new(),
        }
    }
}

impl From<records::tweets::Tweet> for TweetData {
    fn from(tweet: records::tweets::Tweet) -> Self {
        Self {
            key: Key::new(tweet.id, ResourceType::Tweet),
            attributes: TweetAttributes {
                title: tweet.title,
                description: tweet.description,
                signature: tweet.signature,
                timestamp: tweet.timestamp,
                hash: tweet.hash,
            },
            relationships: TweetRelations {
                author: Relation {
                    data: Key::new(tweet.user_id, ResourceType::User),
                },
            },
        }
    }
}

impl From<records::tweets::Tweet> for Tweet {
    fn from(tweet: records::tweets::Tweet) -> Self {
        Self {
            data: TweetData::from(tweet),
        }
    }
}

impl From<Vec<records::tweets::Tweet>> for TweetList {
    fn from(tasks: Vec<records::tweets::Tweet>) -> Self {
        Self {
            data: tasks.into_iter().map(|raw| TweetData::from(raw)).collect(),
        }
    }
}

impl Reply for Tweet {
    fn into_response(self) -> warp::reply::Response {
        http::Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}

impl Reply for TweetList {
    fn into_response(self) -> warp::reply::Response {
        http::Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}
