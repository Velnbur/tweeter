use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

use super::key::Key;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserAttributes {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    #[serde(rename = "type")]
    pub _type: ResourceType,
    pub attributes: CreateUserAttributes,
}

impl CreateUser {
    pub fn new(username: String) -> Self {
        Self {
            _type: ResourceType::User,
            attributes: CreateUserAttributes { username },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub data: CreateUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub key: Key,
    pub attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub data: User,
}
