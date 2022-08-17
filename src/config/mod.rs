mod db;
mod logger;
mod raw;
mod server;
mod storage;

use std::fs;
use std::net::SocketAddrV4;
use thiserror::Error;

pub struct Config {
    pub db: sqlx::PgPool,
    pub server: SocketAddrV4,
    pub storage: s3::Bucket,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open config file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("failed to deserialize file: {0}")]
    DeserializeError(#[from] toml::de::Error),
    #[error("failed to parse file: {0}")]
    ParseError(#[from] raw::Error),
}

impl Config {
    pub async fn from_file(filename: String) -> Result<Self, Error> {
        let content = fs::read_to_string(filename).map_err(Error::FileError)?;

        let raw_config: raw::Raw = toml::from_str(&content).map_err(Error::DeserializeError)?;

        raw_config.parse().await.map_err(Error::ParseError)
    }
}
