use mobc_postgres::tokio_postgres;
use mobc_postgres::tokio_postgres::Row;
use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};

use crate::db::{self, Pool};
use crate::records::tables::Users;

pub struct User {
    pub id: i64,
    pub public_key: String,
}

impl User {
    pub async fn create(public_key: String, db: &Pool) -> Result<Self, db::errors::Error> {
        let con = db::get_con(db).await?;

        let (query, values) = Query::insert()
            .into_table(Users::Table)
            .columns([Users::PublicKey])
            .values_panic(vec![public_key.into()])
            .returning_all()
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await?;

        let row = rows.get(0).unwrap(); // TODO:
        Ok(Self::from(row))
    }

    pub async fn find(id: i64, db: &Pool) -> Result<Option<Self>, db::errors::Error> {
        let con = db::get_con(db).await?;

        let (query, values) = Query::select()
            .from(Users::Table)
            .columns([Users::ID, Users::PublicKey])
            .limit(1)
            .and_where(Expr::col(Users::ID).eq(id))
            .build(PostgresQueryBuilder);

        let rows = con.query(query.as_str(), &values.as_params())
            .await?;

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
            id: r.get(0),
            public_key: r.get(1),
        }
    }
}