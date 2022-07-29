use mobc_postgres::tokio_postgres;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    PoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    QueryError(#[from] tokio_postgres::Error),
    #[error("error creating table: {0}")]
    MigrateError(tokio_postgres::Error),
}