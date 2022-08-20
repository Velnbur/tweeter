use crate::records;
use tweeter_schemas::key::Key;
use tweeter_schemas::relation::Relation;
use tweeter_schemas::resource_type::ResourceType;
use tweeter_schemas::tweets::*;
use tweeter_schemas::users::*;

impl Into<records::tweets::Tweet> for CreateTweetRequest {
    fn into(self) -> records::tweets::Tweet {
        records::tweets::Tweet {
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

impl From<records::tweets::Tweet> for Tweet {
    fn from(tweet: records::tweets::Tweet) -> Self {
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

impl From<records::tweets::Tweet> for TweetResponse {
    fn from(tweet: records::tweets::Tweet) -> Self {
        Self {
            data: Tweet::from(tweet),
        }
    }
}

impl Into<records::users::User> for CreateUserRequest {
    fn into(self) -> records::users::User {
        records::users::User {
            public_key: String::new(),
            username: self.data.attributes.username,
            image_url: None,
        }
    }
}

impl From<records::users::User> for User {
    fn from(user: records::users::User) -> Self {
        Self {
            key: Key::new(user.public_key, ResourceType::User),
            attributes: UserAttributes {
                username: user.username,
                image_url: user.image_url,
            },
        }
    }
}

impl From<records::users::User> for UserResponse {
    fn from(user: records::users::User) -> Self {
        Self {
            data: User::from(user),
        }
    }
}
