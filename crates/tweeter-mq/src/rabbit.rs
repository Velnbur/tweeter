use std::future::Future;

use lapin::{
    options::{BasicAckOptions, BasicGetOptions, BasicPublishOptions},
    BasicProperties,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Consumer, Publisher};

#[derive(Clone)]
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
pub enum BaseError {
    #[error("failed to serialize value: {0}")]
    SerializeError(#[from] serde_json::Error),
    #[error("failed to interact with rabbit: {0}")]
    QueueError(#[from] lapin::Error),
}

#[derive(Error, Debug)]
pub enum Error<E>
where
    E: std::error::Error,
{
    #[error(transparent)]
    Base(#[from] BaseError),
    #[error(transparent)]
    ConsumeError(E),
}

#[async_trait::async_trait]
impl<T> Publisher<T> for RabbitChannel
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'static,
{
    type Error = BaseError;

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
}

#[async_trait::async_trait]
impl<T, CE> Consumer<T, CE> for RabbitChannel
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'static,
    CE: std::error::Error,
{
    type Error = Error<CE>;

    async fn consume<F, Fut>(&self, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<(), CE>> + Send + 'static,
    {
        let delivery = self
            .chan
            .basic_get(&self.q_name, BasicGetOptions::default())
            .await
            .map_err(BaseError::QueueError)?;

        if let Some(message) = delivery {
            let parsed: T = serde_json::from_slice(message.data.as_slice())
                .map_err(BaseError::SerializeError)?;

            f(parsed).await.map_err(Error::ConsumeError)?;

            message
                .ack(BasicAckOptions::default())
                .await
                .map_err(BaseError::QueueError)?;
        }

        Ok(())
    }
}
