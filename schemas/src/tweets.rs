use serde::{Deserialize, Serialize};

use super::key::Key;
use super::relation::Relation;
use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetAttributes {
    pub text: String,
    pub timestamp: i32,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetRelations {
    pub author: Relation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<Relation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    #[serde(flatten)]
    pub key: Key,

    pub attributes: TweetAttributes,
    pub relationships: TweetRelations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetResponse {
    pub data: Tweet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetListResponse {
    pub data: Vec<Tweet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetAttributes {
    pub text: String,
    pub timestamp: i32,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweet {
    #[serde(rename = "type")]
    _type: ResourceType,
    pub attributes: CreateTweetAttributes,
}

impl CreateTweet {
    pub fn new(text: String, timestamp: i32, signature: String) -> Self {
        Self {
            _type: ResourceType::Tweet,
            attributes: CreateTweetAttributes {
                text,
                timestamp,
                signature,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub data: CreateTweet,
}
