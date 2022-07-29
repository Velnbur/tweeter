use mobc_postgres::tokio_postgres;
use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::db;
use crate::db::Pool;
use crate::records::tables::Tasks;

pub mod migrations;
mod tables;
mod users;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i16)]
pub enum TaskPriority {
    Normal = 1,
    High,
    Urgent,
}

pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
}

impl Task {
    pub async fn create(
        title: String,
        desc: String,
        priority: TaskPriority,
        db: &Pool,
    ) -> Result<Task, db::errors::Error> {

        let con = db::get_con(db).await?;

        let (query, values) = Query::insert()
            .into_table(Tasks::Table)
            .columns([Tasks::Title, Tasks::Description, Tasks::Priority])
            .values_panic(vec![title.into(), desc.into(), priority.into()])
            .returning_all()
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await?;

        let row = rows.get(0).unwrap(); // TODO:
        Ok(Self::from(row))
    }

    pub async fn find(id: i64, db: &Pool) -> Result<Option<Task>, db::errors::Error> {
        let con = db::get_con(db).await?;

        let (query, values) = Query::select()
            .from(Tasks::Table)
            .columns([Tasks::ID, Tasks::Title, Tasks::Description, Tasks::Priority])
            .limit(1)
            .and_where(Expr::col(Tasks::ID).eq(id))
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

    pub async fn select(db: &Pool) -> Result<Vec<Task>, db::errors::Error> {
        let con = db::get_con(db).await?;

        let (query, values) = Query::select()
            .from(Tasks::Table)
            .columns([Tasks::ID, Tasks::Title, Tasks::Description, Tasks::Priority])
            .build(PostgresQueryBuilder);

        let rows = con
            .query(query.as_str(), &values.as_params())
            .await
            .map_err(db::errors::Error::QueryError)?;

        Ok(rows.into_iter().map(|row| Self::from(&row)).collect())
    }
}

impl From<TaskPriority> for Value {
    fn from(t: TaskPriority) -> Self {
        match t {
            TaskPriority::Normal => Self::SmallInt(Some(1)),
            TaskPriority::High => Self::SmallInt(Some(2)),
            TaskPriority::Urgent => Self::SmallInt(Some(3)),
        }
    }
}

impl TryFrom<i16> for TaskPriority {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Normal),
            2 => Ok(Self::High),
            3 => Ok(Self::Urgent),
            _ => Err(()),
        }
    }
}

impl From<&tokio_postgres::Row> for Task {
    fn from(r: &tokio_postgres::Row) -> Self {
        Self {
            id: r.get(0),
            title: r.get(1),
            description: r.get(2),
            priority: TaskPriority::try_from(r.get::<usize, i16>(3))
                .unwrap_or(TaskPriority::Normal),
        }
    }
}
