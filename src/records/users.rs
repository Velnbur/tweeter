use mobc_postgres::tokio_postgres::error::SqlState;
use mobc_postgres::tokio_postgres::Row;
use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};

use crate::db::Pool;
use crate::records::tables::Users;

use super::errors::Errors;

pub struct User {
    pub public_key: String,
    pub username: String,
    pub image_url: Option<String>,
}

impl User {
    pub async fn create(self, db: &Pool) -> Result<Self, Errors> {
        let con = db.get().await?;

        let (query, values) = Query::insert()
            .into_table(Users::Table)
            .columns([Users::PublicKey, Users::Username])
            .values_panic(vec![self.public_key.into(), self.username.into()])
            .returning_all()
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await
            .map_err(|err| match err.code().unwrap() {
                &SqlState::UNIQUE_VIOLATION => Errors::InvalidUsername,
                _ => Errors::QueryError(err),
            })?;

        let row = rows.get(0).unwrap(); // TODO: Something went totally wrong
        Ok(Self::from(row))
    }

    pub async fn find(pub_key: String, db: &Pool) -> Result<Option<Self>, Errors> {
        let con = db.get().await?;

        let (query, values) = Query::select()
            .from(Users::Table)
            .columns([Users::PublicKey, Users::Username, Users::ImageURL])
            .limit(1)
            .and_where(Expr::col(Users::PublicKey).eq(pub_key))
            .build(PostgresQueryBuilder);

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        let row = match rows.get(0) {
            Some(val) => val,
            None => return Ok(None),
        };

        log::debug!("Row: {:?}", row);
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
