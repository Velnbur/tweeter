use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("username is used")]
    InvalidUsername,
    #[error("row not found")]
    NotFound,
    #[error("failed to query {0}")]
    Query(#[from] sea_query::error::Error),
    #[error("failed to establish connection {0}")]
    Database(#[from] sqlx::Error),
}
