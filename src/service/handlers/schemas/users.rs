use serde::{Deserialize, Serialize};
use crate::records;
use crate::service::handlers::schemas::resource_type::ResourceType;

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

impl Into<records::users::User> for CreateUser {
    fn into(self) -> records::users::User {
        records::users::User {
            public_key: "".to_string(),
            username: self.data.attributes.username,
            image_url: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    pub image_url: String,
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

impl From<records::users::User> for UserData {
    fn from(user: records::users::User) -> Self {
        Self {
            key: Key::new(user.public_key, ResourceType::User),
            attributes: UserAttributes {
                username: user.username,
                image_url: user.image_url,
            }
        }
    }
}

impl From<records::users::User> for User {
    fn from(user: records::users::User) -> Self {
        Self {
            data: UserData::from(user)
        }
    }
}
