use async_trait::async_trait;

use tweeter_schemas::query::Pagination;

use super::errors::Errors;

#[async_trait]
pub trait Repo<T> {
    fn new<'a>(pool: &'a sqlx::PgPool) -> Self;

    async fn get(&mut self) -> Result<T, Errors>;
    async fn update(&mut self, row: T) -> Result<T, Errors>;
    async fn select(&mut self) -> Result<Vec<T>, Errors>;
    async fn insert(&mut self, row: T) -> Result<T, Errors>;
}

pub trait FilterByPubKey {
    fn where_pub_key(&mut self, pub_key: String) -> &mut Self;
    fn where_pub_keys(&mut self, pub_keys: Vec<String>) -> &mut Self;
}

pub trait FilterById {
    fn where_id(&mut self, id: i64) -> &mut Self;
}

pub trait PaginationTrait {
    fn pages(&mut self, pagination: &Pagination) -> &mut Self;
}
