use self::tables::Tasks;
use num_derive::FromPrimitive;
use sea_query::{Expr, PostgresDriver, PostgresQueryBuilder, Query};
use serde::{Deserialize, Serialize};

use tokio_postgres::Row;

pub mod migrate;
pub mod tables;

#[derive(Debug, FromPrimitive, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Normal = 1,
    High,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub priority: TaskPriority,
}

impl From<&Row> for Task {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            priority: num::FromPrimitive::from_i64(row.get("priority")).unwrap(),
        }
    }
}

impl Task {
    pub async fn find(
        id: i64,
        db: tokio_postgres::Client,
    ) -> Result<Option<Task>, tokio_postgres::Error> {
        let (query, values) = Query::select()
            .columns([Tasks::ID, Tasks::Title, Tasks::Description, Tasks::Priority])
            .from(Tasks::Table)
            .and_where(Expr::col(tables::Tasks::ID).eq(id))
            .limit(1)
            .build(PostgresQueryBuilder);

        let rows = db.query(query.as_str(), &values.as_params()).await?;
        let row = match rows.get(0) {
            Some(row) => row,
            None => return Ok(None),
        };

        Ok(Some(Task::from(row)))
    }
}
