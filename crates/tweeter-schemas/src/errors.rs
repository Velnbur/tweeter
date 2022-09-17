use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub status: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl Error {
    pub fn new(status: StatusCode, title: String, detail: Option<String>) -> Self {
        Self {
            status: status.to_string(),
            title,
            detail,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Errors {
    pub inner: Vec<Error>,
}
