use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};

use mobc_postgres::tokio_postgres;
use crate::db;
use crate::db::Pool;
use super::tables::Tweets;

pub struct Tweet {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub user_id: String,
}

impl Tweet {
    pub async fn create(
        title: String,
        desc: String,
        user_id: String,
        db: &Pool,
    ) -> Result<Self, db::errors::Error> {

        let con = db::get_con(db).await?;

        let (query, values) = Query::insert()
            .into_table(Tweets::Table)
            .columns([Tweets::Title, Tweets::Description, Tweets::UserID])
            .values_panic(vec![title.into(), desc.into(), user_id.into()])
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
            .from(Tweets::Table)
            .columns([Tweets::ID, Tweets::Title, Tweets::Description, Tweets::UserID])
            .limit(1)
            .and_where(Expr::col(Tweets::ID).eq(id))
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await
            .map_err(db::errors::Error::QueryError)?;

        let row = match rows.get(0) {
            Some(val) => val,
            None => return Ok(None),
        };
        Ok(Some(Self::from(row)))
    }

    pub async fn select(db: &Pool) -> Result<Vec<Self>, db::errors::Error> {
        let con = db::get_con(db).await?;

        let (query, values) = Query::select()
            .from(Tweets::Table)
            .columns([Tweets::ID, Tweets::Title, Tweets::Description, Tweets::UserID])
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await
            .map_err(db::errors::Error::QueryError)?;

        Ok(rows.into_iter().map(|row| Self::from(&row)).collect())
    }
}

impl From<&tokio_postgres::Row> for Tweet {
    fn from(r: &tokio_postgres::Row) -> Self {
        Self {
            id: r.get(0),
            title: r.get(1),
            description: r.get(2),
            user_id: r.get(3),
        }
    }
}
