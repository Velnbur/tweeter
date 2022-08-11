use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysAttributes {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysData {
    #[serde(rename = "type")]
    _type: ResourceType,
    pub attributes: AuthKeysAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeys {
    pub data: AuthKeysData,
}

impl AuthKeys {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            data: AuthKeysData {
                _type: ResourceType::AuthKeys,
                attributes: AuthKeysAttributes {
                    private_key,
                    public_key,
                },
            },
        }
    }
}
