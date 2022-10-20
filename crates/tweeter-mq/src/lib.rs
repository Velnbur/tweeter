pub mod rabbit;

use std::future::Future;

use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Publisher<T>
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'a,
{
    type Error: std::error::Error;

    async fn publish(&self, value: T) -> Result<(), Self::Error>;
}

#[async_trait::async_trait]
pub trait Consumer<T, CE>
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'a,
    CE: std::error::Error,
{
    type Error: std::error::Error;

    async fn consume<F, Fut>(&self, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(T) -> Fut + Send,
        Fut: Future<Output = Result<(), CE>> + Send + 'static;
}
