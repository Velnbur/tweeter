use std::net::AddrParseError;

use serde::Deserialize;
use thiserror::Error;
use tweeter_config::{
    db::DB,
    logger::{self, Logger},
    Parseable,
};

use super::{server::Server, storage::Storage};
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
    #[error("failed to parse server params: {0}")]
    ServerError(#[from] AddrParseError),
}

impl Raw {
    pub async fn parse(self) -> Result<Config, Error> {
        self.logger.parse().await?;

        Ok(Config {
            server: self.server.parse()?,
            db: self.db.parse().await?,
            storage: self.storage.parse().await?,
        })
    }
}
