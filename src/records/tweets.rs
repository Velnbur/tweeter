use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};

use super::{errors::Errors, tables::Tweets};
use crate::db::Pool;
use mobc_postgres::tokio_postgres;

#[derive(Clone, Debug)]
pub struct Tweet {
    pub id: i64,
    pub text: String,
    pub timestamp: i32,
    pub user_id: String,
    pub signature: String,
    pub hash: Option<String>,
    pub prev_id: Option<i64>,
}

impl Tweet {
    pub async fn create(self, db: &Pool) -> Result<Self, Errors> {
        let con = db.get().await?;

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

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        let row = rows.get(0).unwrap(); // TODO:
        Ok(Self::from(row))
    }

    pub async fn find(id: i64, db: &Pool) -> Result<Option<Self>, Errors> {
        let con = db.get().await?;

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

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        let row = match rows.get(0) {
            Some(val) => val,
            None => return Ok(None),
        };
        Ok(Some(Self::from(row)))
    }

    pub async fn select(db: &Pool) -> Result<Vec<Self>, Errors> {
        let con = db.get().await?;

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
            .build(PostgresQueryBuilder);

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        Ok(rows.into_iter().map(|row| Self::from(&row)).collect())
    }

    pub async fn update(self, db: &Pool) -> Result<Self, Errors> {
        let con = db.get().await?;

        let (query, values) = Query::update()
            .table(Tweets::Table)
            .values(vec![
                (Tweets::Text, self.text.into()),
                (Tweets::Timestamp, self.timestamp.into()),
                (Tweets::UserId, self.user_id.into()),
                (Tweets::Signature, self.signature.into()),
                (Tweets::Hash, self.hash.into()),
                (Tweets::PreviousId, self.prev_id.into()),
            ])
            .and_where(Expr::col(Tweets::Id).eq(self.id))
            .returning_all()
            .build(PostgresQueryBuilder);

        let rows = con.query(query.as_str(), &values.as_params()).await?;

        let row = rows.get(0).unwrap();
        Ok(Self::from(row))
    }
}

impl From<&tokio_postgres::Row> for Tweet {
    fn from(r: &tokio_postgres::Row) -> Self {
        Self {
            id: r.get(0),
            text: r.get(1),
            timestamp: r.get(2),
            user_id: r.get(3),
            signature: r.get(4),
            hash: r.get(5),
            prev_id: r.get(6),
        }
    }
}
