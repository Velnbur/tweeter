use sea_query::{ColumnDef, PostgresQueryBuilder, Table};
use mobc_postgres::tokio_postgres;
use crate::db::Connection;
use crate::records::tables::Tasks;

pub async fn migrate(db: Connection) -> Result<(), tokio_postgres::Error> {
    migrate_tasks(db).await?;
    Ok(())
}

async fn migrate_tasks(db: Connection) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .table(Tasks::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Tasks::ID)
                .big_integer()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Tasks::Title).text().not_null())
        .col(ColumnDef::new(Tasks::Description).text().not_null())
        .col(ColumnDef::new(Tasks::Priority).small_integer().not_null())
        .build(PostgresQueryBuilder);

    db.batch_execute(&sql).await?;
    Ok(())
}
