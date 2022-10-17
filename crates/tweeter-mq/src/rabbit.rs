use lapin::{
    options::{BasicAckOptions, BasicGetOptions, BasicPublishOptions},
    BasicProperties,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Channel;

pub struct RabbitChannel {
    chan: lapin::Channel,
    q_name: String,
}

impl RabbitChannel {
    pub fn new(q_name: String, chan: lapin::Channel) -> Self {
        Self { q_name, chan }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to serialize value: {0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("failed to interact with rabbit: {0}")]
    QueueError(#[from] lapin::Error),
}

#[async_trait::async_trait]
impl<T> Channel<T> for RabbitChannel
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'a,
{
    type Error = Error;

    async fn publish(&self, value: T) -> Result<(), Self::Error> {
        let payload = serde_json::to_string(&value)?;

        self.chan
            .basic_publish(
                "",
                &self.q_name,
                BasicPublishOptions::default(),
                payload.as_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(())
    }

    async fn consume<F>(&self, f: F) -> Result<(), Self::Error>
    where
        F: Fn(T) -> Result<(), Error> + Send,
    {
        let delivery = self
            .chan
            .basic_get(&self.q_name, BasicGetOptions::default())
            .await?;

        if let Some(message) = delivery {
            let parsed: T = serde_json::from_slice(message.data.as_slice())?;

            f(parsed)?;

            message.ack(BasicAckOptions::default()).await?;
        }

        Ok(())
    }
}
