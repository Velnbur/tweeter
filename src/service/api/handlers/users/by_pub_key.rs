use axum::{extract::Path, response::IntoResponse, Extension, Json};
use thiserror::Error;

use crate::{
    db,
    records::users::User as UserRecord,
    service::api::{errors::ErrorResponse, schemas::users::User as UserSchema},
};

pub async fn by_pub_key(
    Path(pub_key): Path<String>,
    Extension(pool): Extension<db::Pool>,
) -> Result<impl IntoResponse, ByPubKeyError> {
    let user = UserRecord::find(pub_key, &pool)
        .await
        .map_err(|err| {
            log::error!("Failed to get user: {err}");
            ByPubKeyError::Database
        })?
        .ok_or(ByPubKeyError::UserNotFound)?;

    Ok(Json(UserSchema::from(user)))
}

#[derive(Error, Debug)]
pub enum ByPubKeyError {
    #[error("user not found")]
    UserNotFound,
    #[error("databse error")]
    Database,
}

impl IntoResponse for ByPubKeyError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::UserNotFound => ErrorResponse::NotFound(self.to_string()),
            Self::Database => ErrorResponse::InternalError,
        };
        resp.into_response()
    }
}
