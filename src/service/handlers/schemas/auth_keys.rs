use crate::service::handlers::schemas::resource_type::ResourceType;

use serde::{Deserialize, Serialize};
use warp::hyper::header::CONTENT_TYPE;
use warp::hyper::http;
use warp::hyper::Body;
use warp::hyper::StatusCode;
use warp::Reply;

use super::JSON_CONTENT_TYPE;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysAttributes {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeysData {
    #[serde(rename = "type")]
    _type: ResourceType,
    pub attributes: AuthKeysAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeys {
    pub data: AuthKeysData,
}

impl AuthKeys {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            data: AuthKeysData {
                _type: ResourceType::AuthKeys,
                attributes: AuthKeysAttributes {
                    private_key,
                    public_key,
                },
            },
        }
    }
}

impl Reply for AuthKeys {
    fn into_response(self) -> warp::reply::Response {
        http::Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap()
    }
}
