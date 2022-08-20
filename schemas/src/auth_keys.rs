use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysAttributes {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeys {
    #[serde(rename = "type")]
    _type: ResourceType,
    pub attributes: AuthKeysAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysResponse {
    pub data: AuthKeys,
}

impl AuthKeysResponse {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            data: AuthKeys {
                _type: ResourceType::AuthKeys,
                attributes: AuthKeysAttributes {
                    private_key,
                    public_key,
                },
            },
        }
    }
}
