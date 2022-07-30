use serde::{Deserialize, Serialize};
use crate::records;
use crate::service::handlers::schemas::resource_type::ResourceType;

use super::key::Key;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(flatten)]
    pub key: Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub data: UserData,
}

impl From<records::users::User> for UserData {
    fn from(user: records::users::User) -> Self {
        Self {
            key: Key::new(user.public_key, ResourceType::User),
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
