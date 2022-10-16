use lapin::{options::BasicPublishOptions, BasicProperties};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Channel;

pub struct RabbitChannel {
    chan: lapin::Channel,
    q_name: String,
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
    for<'a> T: Deserialize<'a> + Serialize,
{
    type Error = Error;

    async fn publish(&mut self, value: T) -> Result<(), Self::Error> {
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
            .await;

        Ok(())
    }

    async fn consume(&mut self) -> Result<T, Self::Error> {
        todo!()
    }
}
