use serde::{Deserialize, Serialize};
use warp::http;
use warp::http::header::CONTENT_TYPE;
use warp::{hyper::Body, hyper::StatusCode, reply::Response};

use super::JSON_CONTENT_TYPE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub status: String,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Errors {
    pub errors: Vec<Error>,
}

impl Error {
    pub fn new(status: StatusCode, title: String, detail: String) -> Self {
        Self {
            status: status.to_string(),
            title,
            detail,
        }
    }
}

impl Errors {
    fn new_error_response(status: StatusCode, title: String, detail: String) -> Response {
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

    pub fn internal_error(detail: String) -> Response {
        Self::new_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
            detail,
        )
    }

    pub fn not_found(detail: String) -> Response {
        Self::new_error_response(StatusCode::NOT_FOUND, String::from("Not Found"), detail)
    }

    pub fn bad_request(detail: String) -> Response {
        Self::new_error_response(StatusCode::BAD_REQUEST, String::from("Bad Request"), detail)
    }

    pub fn forbidden() -> Response {
        Self::new_error_response(
            StatusCode::FORBIDDEN,
            String::from("Forbidden"),
            String::from(""),
        )
    }

    pub fn unauthorized() -> Response {
        Self::new_error_response(
            StatusCode::UNAUTHORIZED,
            String::from("Forbidden"),
            String::from(""),
        )
    }

    pub fn conflict() -> Response {
        Self::new_error_response(
            StatusCode::UNAUTHORIZED,
            String::from("Forbidden"),
            String::from(""),
        )
    }
}
