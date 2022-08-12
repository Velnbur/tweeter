use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::service::api::schemas::errors::Error;

pub enum ErrorResponse {
    InternalError,
    BadRequest(String),
    Unauthorized,
    Forbidden(String),
    Conflict(String),
    NotFound(String),
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let res = match self {
            Self::InternalError => Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
                None,
            ),
            Self::BadRequest(s) => {
                Error::new(StatusCode::BAD_REQUEST, "Bad Request".to_string(), Some(s))
            }
            Self::Conflict(s) => Error::new(StatusCode::CONFLICT, "Conflict".to_string(), Some(s)),
            Self::Unauthorized => {
                Error::new(StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None)
            }
            Self::Forbidden(s) => {
                Error::new(StatusCode::FORBIDDEN, "Forbidden".to_string(), Some(s))
            }
            Self::NotFound(s) => {
                Error::new(StatusCode::NOT_FOUND, "Not Found".to_string(), Some(s))
            }
        };
        Json(res).into_response()
    }
}
