pub mod rabbit;

use std::future::Future;

use serde::{Deserialize, Serialize};

/// Trait for Message brocker producers/publishers
#[async_trait::async_trait]
pub trait Producer<T>: Clone
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'a,
{
    type Error: std::error::Error;

    async fn publish(&self, value: T) -> Result<(), Self::Error>;
}

/// Trait for Message brocker consumers
#[async_trait::async_trait]
pub trait Consumer<T>: Clone
where
    for<'a> T: Deserialize<'a> + Serialize + Send + 'a,
{
    type Error: std::error::Error;

    /// Block current execution and run passed function inside loop.
    /// for each delivered message. If there were no errors, ackwnoledge it.
    async fn consume<F, Fut, CE>(&self, f: F) -> Result<(), Self::Error>
    where
        F: Fn(T) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), CE>> + Send + 'static,
        // The error that will be returned by given closure
        CE: std::error::Error;

    /// Get and parse message from queue as `T`. Autoackwnoledge it
    /// before returning `T` from method if there were no errors after
    /// parsing.
    async fn get(&self) -> Result<Option<T>, Self::Error>;
}
