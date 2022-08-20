use serde::Deserialize;
use thiserror::Error;

use super::{
    db::DB,
    logger::{self, Logger},
    server::Server,
    storage::Storage,
};
use crate::config::Config;

#[derive(Deserialize)]
pub(super) struct Raw {
    pub server: Server,
    pub db: DB,
    pub logger: Logger,
    pub storage: Storage,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse db config: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("failed to parse storage config: {0}")]
    StorageError(#[from] s3::error::S3Error),
    #[error("failed to init logger: {0}")]
    LoggerError(#[from] logger::Error),
}

impl Raw {
    pub async fn parse(self) -> Result<Config, Error> {
        self.logger.init().map_err(Error::LoggerError)?;

        Ok(Config {
            server: self.server.into(),
            db: self.db.parse().await.map_err(Error::DbError)?,
            storage: self.storage.parse().await.map_err(Error::StorageError)?,
        })
    }
}
