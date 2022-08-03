use serde::Serialize;
use serde_json::json;
use warp::{http, Reply};
use warp::http::header::CONTENT_TYPE;
use warp::hyper::{Body, StatusCode};
use warp::reply::Response;

const JSON_CONTENT_TYPE: &str = "application/vnd.api+json";

pub fn json<T: Serialize>(str: T) -> Response {
    warp::reply::json(&str).into_response()
}

fn new_error_response(status: StatusCode, title: String, detail: String) -> Response {
    http::Response::builder()
        .status(status)
        .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(
            json!({
                    "status": status.to_string(),
                    "title": title,
                    "detail": detail
                })
                .to_string(),
        ))
        .unwrap()
}

pub fn internal_error(detail: String) -> Response {
    new_error_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("Internal Server Error"),
        detail,
    )
}

pub fn not_found(detail: String) -> Response {
    new_error_response(
        StatusCode::NOT_FOUND,
        String::from("Not Found"),
        detail,
    )
}

pub fn bad_request(detail: String) -> Response {
    new_error_response(
        StatusCode::BAD_REQUEST,
        String::from("Bad Request"),
        detail,
    )
}

pub fn forbidden() -> Response {
    new_error_response(
        StatusCode::FORBIDDEN,
        String::from("Forbidden"),
        String::from(""),
    )
}

pub fn unauthorized() -> Response {
    new_error_response(
        StatusCode::UNAUTHORIZED,
        String::from("Forbidden"),
        String::from(""),
    )
}

pub fn conflict() -> Response {
    new_error_response(
        StatusCode::UNAUTHORIZED,
        String::from("Forbidden"),
        String::from(""),
    )
}
