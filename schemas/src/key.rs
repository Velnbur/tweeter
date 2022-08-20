use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub id: String,
    #[serde(rename = "type")]
    _type: ResourceType,
}

impl Key {
    pub fn new<T: ToString>(id: T, _type: ResourceType) -> Self {
        Self { id: id.to_string(), _type }
    }
}