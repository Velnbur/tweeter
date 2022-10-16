pub mod db;
pub mod logger;
pub mod rabbit;

#[async_trait::async_trait]
pub trait Parseable {
    type Error;
    type Value;

    async fn parse(self) -> Result<Self::Value, Self::Error>;
}
