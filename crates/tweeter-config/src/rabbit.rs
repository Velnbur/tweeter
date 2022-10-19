use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties};
use serde::Deserialize;

use crate::Parseable;

#[derive(Deserialize)]
pub struct Rabbit {
    pub uri: String,
}

#[async_trait::async_trait]
impl Parseable for Rabbit {
    type Error = lapin::Error;
    type Value = lapin::Channel;

    async fn parse(self) -> Result<Self::Value, Self::Error> {
        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let connection = Connection::connect(self.uri.as_str(), options).await?;

        connection.create_channel().await
    }
}

#[derive(Deserialize)]
pub struct RabbitChannel {
    #[serde(flatten)]
    pub inner: Rabbit,
    pub queue: String,
}

#[async_trait::async_trait]
impl Parseable for RabbitChannel {
    type Error = lapin::Error;
    type Value = tweeter_mq::rabbit::RabbitChannel;

    async fn parse(self) -> Result<Self::Value, Self::Error> {
        let channel = self.inner.parse().await?;

        let _queue = channel
            .queue_declare(
                &self.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(tweeter_mq::rabbit::RabbitChannel::new(self.queue, channel))
    }
}
