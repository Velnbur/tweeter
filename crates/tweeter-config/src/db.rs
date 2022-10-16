use serde::Deserialize;

use crate::Parseable;

#[derive(Deserialize)]
pub struct DB {
    pub url: String,
}

#[async_trait::async_trait]
impl Parseable for DB {
    type Value = sqlx::PgPool;
    type Error = sqlx::Error;

    async fn parse(self) -> Result<Self::Value, Self::Error> {
        sqlx::PgPool::connect(&self.url).await
    }
}
