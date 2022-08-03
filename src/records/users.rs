use mobc_postgres::tokio_postgres;
use mobc_postgres::tokio_postgres::error::SqlState;
use mobc_postgres::tokio_postgres::Row;
use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};
use thiserror::Error;

use crate::db::Pool;
use crate::records::errors::RecordsError;
use crate::records::tables::Users;

pub struct User {
    pub public_key: String,
    pub username: String,
    pub image_url: String,
}

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("username is used")]
    InvalidUsername,
    #[error("query error {0}")]
    QueryError(#[from] tokio_postgres::Error),
    #[error("failed to create connection {0}")]
    ConnectionError(#[from] mobc::Error<tokio_postgres::Error>),
}

impl User {
    pub async fn create(self, db: &Pool) -> Result<Self, UsersError> {
        let con = db.get().await.map_err(UsersError::ConnectionError)?;

        let (query, values) = Query::insert()
            .into_table(Users::Table)
            .columns([Users::PublicKey, Users::Username, Users::ImageURL])
            .values_panic(vec![
                self.public_key.into(),
                self.username.into(),
                self.image_url.into(),
            ])
            .returning_all()
            .build(PostgresQueryBuilder);

        let rows = match con.query(query.as_str(), &values.as_params()).await {
            Ok(r) => r,
            Err(err) => {
                return match err.code().unwrap() {
                    &SqlState::UNIQUE_VIOLATION => Err(UsersError::InvalidUsername),
                    _ => Err(UsersError::QueryError(err)),
                }
            }
        };

        let row = rows.get(0).unwrap(); // TODO:
        Ok(Self::from(row))
    }

    pub async fn find(pub_key: String, db: &Pool) -> Result<Option<Self>, RecordsError> {
        let con = db.get().await?;

        let (query, values) = Query::select()
            .from(Users::Table)
            .columns([Users::PublicKey])
            .limit(1)
            .and_where(Expr::col(Users::PublicKey).eq(pub_key))
            .build(PostgresQueryBuilder);

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        let row = match rows.get(0) {
            Some(val) => val,
            None => return Ok(None),
        };

        Ok(Some(Self::from(row)))
    }
}

impl From<&Row> for User {
    fn from(r: &Row) -> Self {
        Self {
            public_key: r.get(0),
            username: r.get(1),
            image_url: r.get(2),
        }
    }
}
