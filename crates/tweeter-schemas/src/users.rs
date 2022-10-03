use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::resource_type::ResourceType;

use super::key::Key;

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct CreateUserAttributes {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(equal = 45))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

fn validate_user_type(res: &ResourceType) -> Result<(), ValidationError> {
    if *res == ResourceType::User {
        return Ok(());
    }

    Err(ValidationError::new("invalid resource type"))
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    #[validate(custom = "validate_user_type")]
    #[serde(rename = "type")]
    pub _type: ResourceType,
    #[validate]
    pub attributes: CreateUserAttributes,
}

impl CreateUser {
    pub fn new(username: String) -> Self {
        Self {
            _type: ResourceType::User,
            attributes: CreateUserAttributes {
                username,
                public_key: None,
            },
        }
    }

    pub fn with_key(self, key: String) -> Self {
        Self {
            _type: ResourceType::User,
            attributes: CreateUserAttributes {
                public_key: Some(key),
                ..self.attributes
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateUserRequest {
    #[validate]
    pub data: CreateUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAttributes {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(flatten)]
    pub key: Key,
    pub attributes: UserAttributes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub data: User,
}

use tweeter_models::user::User as UserModel;

impl Into<UserModel> for CreateUserRequest {
    fn into(self) -> UserModel {
        let public_key = self.data.attributes.public_key.unwrap_or_default();
        UserModel {
            public_key,
            username: self.data.attributes.username,
            image_url: None,
        }
    }
}

impl From<UserModel> for User {
    fn from(user: UserModel) -> Self {
        Self {
            key: Key::new(user.public_key, ResourceType::User),
            attributes: UserAttributes {
                username: user.username,
                image_url: user.image_url,
            },
        }
    }
}

impl From<UserModel> for UserResponse {
    fn from(user: UserModel) -> Self {
        Self {
            data: User::from(user),
        }
    }
}

impl From<User> for UserModel {
    fn from(value: User) -> Self {
        Self {
            public_key: value.key.id,
            username: value.attributes.username,
            image_url: value.attributes.image_url,
        }
    }
}

impl From<UserResponse> for UserModel {
    fn from(value: UserResponse) -> Self {
        UserModel::from(value.data)
    }
}
