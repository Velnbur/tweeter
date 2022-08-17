use sea_query::{Expr, PostgresQueryBuilder, Query};

use super::errors::Errors;
use super::sea_query_driver_postgres::bind_query_as;
use super::{pagination::Pagination, tables::Tweets};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
    pub timestamp: i32,
    pub user_id: String,
    pub signature: String,
    pub hash: Option<String>,
    pub previous_id: Option<i64>,
}

impl Tweet {
    pub async fn create(self, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::insert()
            .into_table(Tweets::Table)
            .columns([
                Tweets::Text,
                Tweets::Timestamp,
                Tweets::UserId,
                Tweets::Signature,
            ])
            .values_panic(vec![
                self.text.into(),
                self.timestamp.into(),
                self.user_id.into(),
                self.signature.into(),
            ])
            .returning_all()
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }

    pub async fn find(id: i64, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::select()
            .from(Tweets::Table)
            .columns([
                Tweets::Id,
                Tweets::Text,
                Tweets::Timestamp,
                Tweets::UserId,
                Tweets::Signature,
                Tweets::Hash,
                Tweets::PreviousId,
            ])
            .limit(1)
            .and_where(Expr::col(Tweets::Id).eq(id))
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => Errors::NotFound,
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn select(pool: &sqlx::PgPool, pagination: &Pagination) -> Result<Vec<Self>, Errors> {
        let (query, values) = Query::select()
            .from(Tweets::Table)
            .columns([
                Tweets::Id,
                Tweets::Text,
                Tweets::Timestamp,
                Tweets::UserId,
                Tweets::Signature,
                Tweets::Hash,
                Tweets::PreviousId,
            ])
            .limit(pagination.limit)
            .order_by(Tweets::Id, pagination.order.into())
            .offset(pagination.number * pagination.limit)
            .build(PostgresQueryBuilder);

        let rows = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_all(pool)
            .await
            .map_err(Errors::Database)?;

        Ok(rows)
    }

    pub async fn update(self, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::update()
            .table(Tweets::Table)
            .values(vec![
                (Tweets::Text, self.text.into()),
                (Tweets::Timestamp, self.timestamp.into()),
                (Tweets::UserId, self.user_id.into()),
                (Tweets::Signature, self.signature.into()),
                (Tweets::Hash, self.hash.into()),
                (Tweets::PreviousId, self.previous_id.into()),
            ])
            .and_where(Expr::col(Tweets::Id).eq(self.id))
            .returning_all()
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }
}
