use sea_query::{Expr, PostgresQueryBuilder, Query};
use tweeter_models::tweet::Tweet;
use tweeter_schemas::query::Pagination;

use super::sea_query_driver_postgres::bind_query_as;
use super::{errors::Errors, tables::Tweets};

pub struct TweetsRepo<'a> {
    insert: sea_query::InsertStatement,
    select: sea_query::SelectStatement,
    update: sea_query::UpdateStatement,
    pool: &'a sqlx::PgPool,
}

unsafe impl<'a> Send for TweetsRepo<'a> {}

impl<'a> TweetsRepo<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> Self {
        Self {
            insert: Query::insert()
                .into_table(Tweets::Table)
                .columns([
                    Tweets::Text,
                    Tweets::Timestamp,
                    Tweets::UserId,
                    Tweets::Signature,
                ])
                .returning_all()
                .to_owned(),
            select: Query::select()
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
                .to_owned(),
            update: Query::update()
                .table(Tweets::Table)
                .returning_all()
                .to_owned(),
            pool,
        }
    }

    pub fn where_id(&mut self, id: i64) -> &mut Self {
        self.select.and_where(Expr::col(Tweets::Id).eq(id));
        self.update.and_where(Expr::col(Tweets::Id).eq(id));

        self
    }

    pub fn pages(&mut self, pagination: &Pagination) -> &mut Self {
        self.select
            .limit(pagination.limit)
            .order_by(
                Tweets::Id,
                match pagination.order {
                    tweeter_schemas::query::Order::Desc => sea_query::Order::Desc,
                    tweeter_schemas::query::Order::Asc => sea_query::Order::Asc,
                },
            )
            .offset(pagination.number * pagination.limit);

        self
    }

    pub async fn get(&mut self) -> Result<Tweet, Errors> {
        let (query, values) = self.select.limit(1).build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => Errors::NotFound,
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn update(&mut self, tweet: Tweet) -> Result<Tweet, Errors> {
        let (query, values) = self
            .update
            .values(vec![
                (Tweets::Text, tweet.text.into()),
                (Tweets::Timestamp, tweet.timestamp.into()),
                (Tweets::UserId, tweet.user_id.into()),
                (Tweets::Signature, tweet.signature.into()),
                (Tweets::Hash, tweet.hash.into()),
                (Tweets::PreviousId, tweet.previous_id.into()),
            ])
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(self.pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }

    pub async fn select(&mut self) -> Result<Vec<Tweet>, Errors> {
        let (query, values) = self.select.build(PostgresQueryBuilder);

        let rows = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_all(self.pool)
            .await
            .map_err(Errors::Database)?;

        Ok(rows)
    }

    pub async fn insert(&mut self, tweet: Tweet) -> Result<Tweet, Errors> {
        let (query, values) = self
            .insert
            .values(vec![
                tweet.text.into(),
                tweet.timestamp.into(),
                tweet.user_id.into(),
                tweet.signature.into(),
            ])
            .map_err(Errors::Query)?
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, Tweet>(&query), &values)
            .fetch_one(self.pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }
}
