use serde::{Deserialize, Serialize};
use warp::http;
use warp::http::header::CONTENT_TYPE;
use warp::{hyper::Body, hyper::StatusCode, reply::Response};

use super::JSON_CONTENT_TYPE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub status: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Errors {
    pub errors: Vec<Error>,
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

impl Errors {
    fn new_error_response(status: StatusCode, title: String, detail: Option<String>) -> Response {
        http::Response::builder()
            .status(status)
            .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(Body::from(
                serde_json::to_string(&Self {
                    errors: vec![Error::new(status, title, detail)],
                })
                .unwrap(),
            ))
            .unwrap()
    }

    pub fn internal_error(detail: Option<String>) -> Response {
        Self::new_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
            detail,
        )
    }

    pub fn not_found(detail: String) -> Response {
        Self::new_error_response(
            StatusCode::NOT_FOUND,
            String::from("Not Found"),
            Some(detail),
        )
    }

    pub fn unauthorized() -> Response {
        Self::new_error_response(StatusCode::UNAUTHORIZED, String::from("Unauthorized"), None)
    }

    pub fn forbidden() -> Response {
        Self::new_error_response(StatusCode::FORBIDDEN, String::from("Forbidden"), None)
    }

    pub fn conflict() -> Response {
        Self::new_error_response(StatusCode::CONFLICT, String::from("Conflict"), None)
    }
}
