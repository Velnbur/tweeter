use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::postgres::PgDatabaseError;

use super::errors::Errors;
use super::sea_query_driver_postgres::bind_query_as;
use crate::records::tables::Users;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub public_key: String,
    pub username: String,
    pub image_url: Option<String>,
}

impl User {
    pub async fn create(self, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::insert()
            .into_table(Users::Table)
            .columns([Users::PublicKey, Users::Username])
            .returning_all()
            .values(vec![self.public_key.into(), self.username.into()])
            .map_err(Errors::Query)?
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::Database(err) => match err.downcast_ref::<PgDatabaseError>().code() {
                    "23505" => Errors::InvalidUsername,
                    &_ => Errors::Database(sqlx::Error::Database(err)),
                },
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn find(pub_key: String, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::select()
            .from(Users::Table)
            .columns([Users::PublicKey, Users::Username, Users::ImageURL])
            .limit(1)
            .and_where(Expr::col(Users::PublicKey).eq(pub_key))
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => Errors::NotFound,
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn update(self, pool: &sqlx::PgPool) -> Result<Self, Errors> {
        let (query, values) = Query::update()
            .table(Users::Table)
            .values(vec![(Users::ImageURL, self.image_url.into())])
            .and_where(Expr::col(Users::PublicKey).eq(self.public_key))
            .returning_all()
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }
}
