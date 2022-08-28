use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::postgres::PgDatabaseError;
use tweeter_models::user::User;

use super::{errors::Errors, sea_query_driver_postgres::bind_query_as, tables::Users};

pub struct UsersRepo<'a> {
    insert: sea_query::InsertStatement,
    select: sea_query::SelectStatement,
    update: sea_query::UpdateStatement,
    pool: &'a sqlx::PgPool,
}

impl<'a> UsersRepo<'a> {
    pub fn new(pool: &'a sqlx::PgPool) -> &mut Self {
        &mut Self {
            insert: Query::insert()
                .into_table(Users::Table)
                .columns([Users::PublicKey, Users::Username])
                .returning_all()
                .to_owned(),
            select: Query::select()
                .from(Users::Table)
                .columns([Users::PublicKey, Users::Username, Users::ImageURL])
                .to_owned(),
            update: Query::update()
                .table(Users::Table)
                .returning_all()
                .to_owned(),
            pool,
        }
    }

    pub fn where_pub_key(&mut self, pub_key: String) -> &mut Self {
        self.select
            .and_where(Expr::col(Users::PublicKey).eq(pub_key));
        self.update
            .and_where(Expr::col(Users::PublicKey).eq(pub_key));

        self
    }

    pub fn where_pub_keys(&mut self, pub_keys: Vec<String>) -> &mut Self {
        self.select
            .and_where(Expr::col(Users::PublicKey).is_in(pub_keys));
        self.update
            .and_where(Expr::col(Users::PublicKey).is_in(pub_keys));

        self
    }

    pub async fn get(&mut self) -> Result<User, Errors> {
        let (query, values) = self.select.limit(1).build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => Errors::NotFound,
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn select(&mut self) -> Result<Vec<User>, Errors> {
        let (query, values) = self.select.build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_all(self.pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => Errors::NotFound,
                _ => Errors::Database(err),
            })?;

        Ok(row)
    }

    pub async fn insert(&mut self, user: User) -> Result<User, Errors> {
        let (query, values) = self
            .insert
            .values(vec![user.public_key.into(), user.username.into()])
            .map_err(Errors::Query)?
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(self.pool)
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

    pub async fn update(&mut self, user: User) -> Result<User, Errors> {
        let (query, values) = self
            .update
            .values(vec![(Users::ImageURL, user.image_url.into())])
            .build(PostgresQueryBuilder);

        let row = bind_query_as(sqlx::query_as::<_, User>(&query), &values)
            .fetch_one(self.pool)
            .await
            .map_err(Errors::Database)?;

        Ok(row)
    }
}
