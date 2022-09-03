use std::num::ParseIntError;

use serde::{Deserialize, Serialize};

use crate::include::Include;
use crate::users::{User, UserAttributes};

use super::key::Key;
use super::relation::Relation;
use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TweetAttributes {
    pub text: String,
    pub timestamp: i32,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TweetRelations {
    pub author: Relation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<Relation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tweet {
    #[serde(flatten)]
    pub key: Key,

    pub attributes: TweetAttributes,
    pub relationships: TweetRelations,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TweetResponse {
    pub data: Tweet,
    pub include: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TweetListResponse {
    pub data: Vec<Tweet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTweetAttributes {
    pub text: String,
    pub timestamp: i32,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTweetRequest {
    pub data: CreateTweet,
}

use tweeter_models::tweet::Tweet as TweetModel;

impl Into<TweetModel> for CreateTweetRequest {
    fn into(self) -> TweetModel {
        TweetModel {
            id: 0,
            text: self.data.attributes.text,
            signature: self.data.attributes.signature,
            timestamp: self.data.attributes.timestamp,
            hash: None,
            user_id: String::new(),
            previous_id: None,
        }
    }
}

impl From<TweetModel> for Tweet {
    fn from(tweet: TweetModel) -> Self {
        let previous = match tweet.previous_id {
            None => None,
            Some(id) => Some(Relation {
                data: Key::new(id.to_string(), ResourceType::Tweet),
            }),
        };

        Self {
            key: Key::new(tweet.id, ResourceType::Tweet),
            attributes: TweetAttributes {
                text: tweet.text,
                signature: tweet.signature,
                timestamp: tweet.timestamp,
                hash: tweet.hash,
            },
            relationships: TweetRelations {
                author: Relation {
                    data: Key::new(tweet.user_id, ResourceType::User),
                },
                previous,
            },
        }
    }
}

impl From<TweetModel> for TweetResponse {
    fn from(tweet: TweetModel) -> Self {
        Self {
            data: Tweet::from(tweet),
            include: None,
        }
    }
}

impl TryFrom<Tweet> for TweetModel {
    type Error = ParseIntError;

    fn try_from(tweet: Tweet) -> Result<Self, Self::Error> {
        let previous_id = match tweet.relationships.previous {
            Some(val) => Some(val.data.id.parse()?),
            None => None,
        };
        Ok(Self {
            id: tweet.key.id.parse()?,
            text: tweet.attributes.text,
            timestamp: tweet.attributes.timestamp,
            user_id: tweet.relationships.author.data.id,
            signature: tweet.attributes.signature,
            hash: tweet.attributes.hash,
            previous_id,
        })
    }
}

impl TryFrom<TweetResponse> for TweetModel {
    type Error = ParseIntError;

    fn try_from(value: TweetResponse) -> Result<Self, Self::Error> {
        TweetModel::try_from(value.data)
    }
}

impl TryFrom<TweetListResponse> for Vec<TweetModel> {
    type Error = ParseIntError;

    fn try_from(value: TweetListResponse) -> Result<Self, Self::Error> {
        value
            .data
            .into_iter()
            .map(|tweet| TweetModel::try_from(tweet))
            .collect()
    }
}

impl From<Vec<TweetModel>> for TweetListResponse {
    fn from(tweets: Vec<TweetModel>) -> Self {
        Self {
            data: tweets.into_iter().map(|tweet| Tweet::from(tweet)).collect(),
            include: None,
        }
    }
}

use tweeter_models::user::User as UserModel;

impl Include<UserModel> for TweetResponse {
    fn include(&mut self, user: UserModel) -> &mut Self {
        self.include = Some(User {
            key: Key::new(user.public_key, ResourceType::User),
            attributes: UserAttributes {
                username: user.username,
                image_url: user.image_url,
            },
        });

        self
    }
}

impl Include<Vec<UserModel>> for TweetListResponse {
    fn include(&mut self, resource: Vec<UserModel>) -> &mut Self {
        self.include = Some(resource.into_iter().map(|user| User::from(user)).collect());

        self
    }
}
