use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;
use crate::records;

use super::key::Key;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserAttributes {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserData {
    pub attributes: CreateUserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub data: CreateUserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(flatten)]
    pub key: Key,
    pub attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub data: UserData,
}

impl Into<records::users::User> for CreateUser {
    fn into(self) -> records::users::User {
        records::users::User {
            public_key: String::new(),
            username: self.data.attributes.username,
            image_url: None,
        }
    }
}

impl From<records::users::User> for UserData {
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

impl From<records::users::User> for User {
    fn from(user: records::users::User) -> Self {
        Self {
            data: UserData::from(user),
        }
    }
}
