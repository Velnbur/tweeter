use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct DB {
    pub url: String,
}

impl DB {
    pub async fn parse(self) -> Result<sqlx::PgPool, sqlx::Error> {
        sqlx::PgPool::connect(&self.url).await
    }
}
