use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceType {
    Tweet,
    User,
    AuthKeys,
}

pub fn validate_resourse_type(a: ResourceType, b: ResourceType) -> Result<(), ValidationError> {
    if a == b {
        return Ok(());
    }

    Err(ValidationError::new("invalid resourse type"))
}
