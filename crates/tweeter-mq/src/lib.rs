pub mod rabbit;

use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Channel<T>
where
    for<'a> T: Deserialize<'a> + Serialize,
{
    type Error;

    async fn consume(&mut self) -> Result<T, Self::Error>;
    async fn publish(&mut self, value: T) -> Result<(), Self::Error>;
}
