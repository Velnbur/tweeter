pub mod config;
pub mod db;
pub mod logger;
pub mod rabbit;

#[async_trait::async_trait]
pub trait Parseable {
    type Value;

    type Error: std::error::Error;

    async fn parse(self) -> Result<Self::Value, Self::Error>;
}
