use serde_json::json;
use warp::http;
use warp::hyper::{Body, StatusCode};
use warp::Reply;
use warp::reply::Response;

pub struct ResponseError {
    pub status: StatusCode,
    pub title: String,
    pub detail: String,
}

impl ResponseError {
    pub fn new(status: StatusCode, title: String, detail: String) -> Self {
        Self {
            status,
            title,
            detail,
        }
    }
}

impl Reply for ResponseError {
    fn into_response(self) -> Response {
        http::Response::builder()
            .status(self.status)
            .body(Body::from(
                json!({
                    "status": self.status.to_string(),
                    "title": self.title,
                    "detail": self.detail
                })
                .to_string(),
            ))
            .unwrap()
    }
}

pub struct InternalError {
    parent: ResponseError,
}

impl InternalError {
    pub fn new(detail: String) -> Self {
        Self {
            parent: ResponseError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal Server Error"),
                detail,
            ),
        }
    }
}

impl Reply for InternalError {
    fn into_response(self) -> Response {
        self.parent.into_response()
    }
}

pub struct NotFound {
    parent: ResponseError,
}

impl NotFound {
    pub fn new(detail: String) -> Self {
        Self {
            parent: ResponseError::new(
                StatusCode::NOT_FOUND,
                String::from("Not Found"),
                detail,
            ),
        }
    }
}

impl Reply for NotFound {
    fn into_response(self) -> Response {
        self.parent.into_response()
    }
}

pub struct BadRequest {
    parent: ResponseError,
}

impl BadRequest {
    pub fn new(detail: String) -> Self {
        Self {
            parent: ResponseError::new(
                StatusCode::BAD_REQUEST,
                String::from("Bad Request"),
                detail,
            ),
        }
    }
}

impl Reply for BadRequest {
    fn into_response(self) -> Response {
        self.parent.into_response()
    }
}