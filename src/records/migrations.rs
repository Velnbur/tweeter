use crate::db::Connection;
use crate::records::tables::{Tweets, Users};
use mobc_postgres::tokio_postgres;
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, PostgresQueryBuilder, Table};

pub async fn migrate(db: Connection) -> Result<(), tokio_postgres::Error> {
    migrate_users(&db).await?;
    migrate_tweets(&db).await?;
    Ok(())
}

async fn migrate_tweets(db: &Connection) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .table(Tweets::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Tweets::ID)
                .big_integer()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Tweets::Title).text().not_null())
        .col(ColumnDef::new(Tweets::Description).text().not_null())
        .col(ColumnDef::new(Tweets::UserID).text().not_null())
        .foreign_key(
            ForeignKey::create()
                .from(Tweets::Table, Tweets::UserID)
                .to(Users::Table, Users::PublicKey)
                .on_delete(ForeignKeyAction::Cascade)
        )
        .build(PostgresQueryBuilder);

    println!("{}", sql);
    db.batch_execute(&sql).await?;
    Ok(())
}

async fn migrate_users(db: &Connection) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .table(Users::Table)
        .if_not_exists()
        .col(ColumnDef::new(Users::PublicKey).text().primary_key())
        .build(PostgresQueryBuilder);

    println!("{}", sql);
    db.batch_execute(&sql).await?;
    Ok(())
}
