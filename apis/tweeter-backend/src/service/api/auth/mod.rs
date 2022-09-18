use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Credentials, Authorization},
    response::IntoResponse,
    TypedHeader,
};
use thiserror::Error;
use tweeter_auth::verify_signature;

use crate::service::api::errors::ErrorResponse;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken,
    #[error("timestamp is old")]
    InvalidTimestamp,
    #[error("failed to get current time: {0}")]
    TimeError(#[from] SystemTimeError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::InvalidToken | Self::InvalidTimestamp => {
                ErrorResponse::BadRequest(self.to_string())
            }
            AuthError::TimeError(err) => {
                log::error!("Failed to get current time: {err}");
                ErrorResponse::InternalError
            }
        };
        resp.into_response()
    }
}

pub struct Claims {
    pub pub_key: String,
    timestamp: u64,
    signature: String,
}

impl Claims {
    const INTERVAL: u64 = 60 * 5;

    pub fn verify(&self) -> Result<(), AuthError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(AuthError::TimeError)?;

        if now.as_secs() - self.timestamp > Self::INTERVAL {
            return Err(AuthError::InvalidTimestamp);
        }

        let mut msg = String::new();

        msg.push_str(self.timestamp.to_string().as_str());
        msg.push('.');
        msg.push_str(self.pub_key.as_str());

        verify_signature(&msg, &self.signature, &self.pub_key).map_err(|_| AuthError::InvalidToken)
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(craber)) =
            TypedHeader::<Authorization<Craber>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;

        let claims = Claims::try_from(craber)?;

        claims.verify()?;

        Ok(claims)
    }
}

impl TryFrom<Craber> for Claims {
    type Error = AuthError;

    fn try_from(value: Craber) -> Result<Self, Self::Error> {
        let mut parts = value.0.splitn(3, '.');

        let timestamp = parts.next().ok_or(AuthError::InvalidToken)?;
        let pub_key = parts.next().ok_or(AuthError::InvalidToken)?;
        let signature = parts.next().ok_or(AuthError::InvalidToken)?;

        Ok(Self {
            timestamp: timestamp.parse().map_err(|_| AuthError::InvalidToken)?,
            pub_key: pub_key.to_string(),
            signature: signature.to_string(),
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Craber(pub String);

impl Credentials for Craber {
    const SCHEME: &'static str = "Craber";

    fn decode(value: &axum::http::HeaderValue) -> Option<Self> {
        if value.is_empty() {
            return None;
        }
        let inner = match value.to_str() {
            Ok(v) => v,
            Err(_) => return None,
        };
        if inner.len() < Self::SCHEME.len() + 1 {
            return None;
        }
        Some(Self(String::from(&inner[Self::SCHEME.len() + 1..])))
    }

    fn encode(&self) -> axum::http::HeaderValue {
        todo!()
    }
}
