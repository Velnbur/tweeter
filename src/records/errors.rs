use mobc_postgres::tokio_postgres;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecordsError {
    #[error("failed to query {0}")]
    QueryError(#[from] tokio_postgres::Error),
    #[error("failed to establish connection {0}")]
    ConnectionError(#[from] mobc::Error<tokio_postgres::Error>),
}