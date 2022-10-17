pub mod rabbit;

use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Channel<T>
where
    for<'a> T: Deserialize<'a> + Serialize,
{
    type Error: std::error::Error;

    async fn consume<F>(&self, f: F) -> Result<(), Self::Error>
    where
        F: Fn(T) -> Result<(), Self::Error> + Send;

    async fn publish(&self, value: T) -> Result<(), Self::Error>;
}
