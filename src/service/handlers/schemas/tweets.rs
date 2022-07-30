use serde::{Deserialize, Serialize};
use crate::records;

use super::relation::Relation;
use super::resource_type::ResourceType;
use super::key::Key;

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetAttributes {
    pub title: String,
    pub description: String,
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
    pub sign: String,
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

impl From<records::tweets::Tweet> for TweetData {
    fn from(tweet: records::tweets::Tweet) -> Self {
        Self {
            key: Key::new(tweet.id, ResourceType::Tweet),
            attributes: TweetAttributes {
                title: tweet.title,
                description: tweet.description,
            },
            relationships: TweetRelations {
                author: Relation {
                    data: Key::new(tweet.user_id, ResourceType::User)
                }
            }
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
            data: tasks
                .into_iter()
                .map(|raw| TweetData::from(raw))
                .collect(),
        }
    }
}
