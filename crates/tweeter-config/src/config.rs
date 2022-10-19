use serde::Deserialize;
use std::fs;
use thiserror::Error;

use crate::Parseable;

#[derive(Error, Debug)]
pub enum Error<E>
where
    E: std::error::Error + 'static,
{
    #[error("failed to open config file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("failed to deserialize file: {0}")]
    DeserializeError(#[from] toml::de::Error),
    #[error("failed to parse file: {0}")]
    ParseError(E),
}

#[async_trait::async_trait]
pub trait Configer {
    type Raw: Parseable<Value = Box<Self>> + for<'a> Deserialize<'a> + Send;

    async fn from_file(
        filename: String,
    ) -> Result<
        <<Self as Configer>::Raw as Parseable>::Value,
        Error<<<Self as Configer>::Raw as Parseable>::Error>,
    > {
        let content = fs::read_to_string(filename)?;

        let raw_config: Self::Raw = toml::from_str(&content)?;

        raw_config.parse().await.map_err(Error::ParseError)
    }
}
