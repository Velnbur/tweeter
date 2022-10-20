use serde::Deserialize;
use thiserror::Error;
use tweeter_config::{
    db::DB,
    logger::{self, Logger},
    rabbit::RabbitChannel,
    Parseable,
};

use super::Config;

#[derive(Deserialize)]
pub struct Raw {
    pub db: DB,
    pub logger: Logger,
    pub mq: RabbitChannel,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse db: {0}")]
    DB(#[from] sqlx::Error),
    #[error("failed to parse logger: {0}")]
    Logger(#[from] logger::Error),
    #[error("failed to parse rabbit: {0}")]
    MqError(#[from] lapin::Error),
}

#[async_trait::async_trait]
impl Parseable for Raw {
    type Value = Box<Config>;
    type Error = Error;

    async fn parse(self) -> Result<Self::Value, Self::Error> {
        self.logger.parse().await?;

        Ok(Box::new(Config {
            db: self.db.parse().await?,
            mq: self.mq.parse().await?,
        }))
    }
}
