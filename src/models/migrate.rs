use sea_query::{ColumnDef, PostgresQueryBuilder, Table};

use super::{tables::Tasks, TaskPriority};

pub async fn migrate(db: &tokio_postgres::Client) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .if_not_exists()
        .col(
            ColumnDef::new(Tasks::ID)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Tasks::Title).text().not_null())
        .col(ColumnDef::new(Tasks::Description).text().not_null())
        .col(
            ColumnDef::new(Tasks::Priority)
                .enumeration("task_priority", &[1, 2, 3])
                .default(TaskPriority::Normal.to_string()),
        )
        .build(PostgresQueryBuilder);

    db.batch_execute(&sql).await?;
    Ok(())
}
