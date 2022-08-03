pub(super) mod auth_keys;
pub(super) mod errors;
pub(super) mod tweets;
pub(super) mod users;

mod key;
mod relation;
mod resource_type;

const JSON_CONTENT_TYPE: &str = "application/vnd.api+json";
