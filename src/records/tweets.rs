use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};

use super::{errors::Errors, tables::Tweets};
use crate::db::Pool;
use mobc_postgres::tokio_postgres;

#[derive(Clone, Debug)]
pub struct Tweet {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub timestamp: u32,
    pub user_id: String,
    pub signature: String,
    pub hash: Option<String>,
}

impl Tweet {
    pub async fn create(self, db: &Pool) -> Result<Self, Errors> {
        let con = db.get().await?;

        let (query, values) = Query::insert()
            .into_table(Tweets::Table)
            .columns([
                Tweets::Title,
                Tweets::Description,
                Tweets::Timestamp,
                Tweets::UserID,
                Tweets::Signature,
            ])
            .values_panic(vec![
                self.title.into(),
                self.description.into(),
                self.timestamp.into(),
                self.signature.into(),
                self.user_id.into(),
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
                Tweets::ID,
                Tweets::Title,
                Tweets::Description,
                Tweets::Timestamp,
                Tweets::Signature,
                Tweets::Hash,
                Tweets::UserID,
            ])
            .limit(1)
            .and_where(Expr::col(Tweets::ID).eq(id))
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
                Tweets::ID,
                Tweets::Title,
                Tweets::Description,
                Tweets::Timestamp,
                Tweets::Signature,
                Tweets::Hash,
                Tweets::UserID,
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
                (Tweets::Title, self.title.into()),
                (Tweets::Description, self.description.into()),
                (Tweets::Timestamp, self.timestamp.into()),
                (Tweets::Signature, self.signature.into()),
                (Tweets::Hash, self.hash.into()),
                (Tweets::UserID, self.user_id.into()),
            ])
            .and_where(Expr::col(Tweets::ID).eq(self.id))
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
            title: r.get(1),
            description: r.get(2),
            timestamp: r.get(3),
            signature: r.get(4),
            hash: r.get(5),
            user_id: r.get(6),
        }
    }
}
