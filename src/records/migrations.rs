use crate::db::{Connection, Pool};
use crate::records::tables::{Tweets, Users};
use mobc_postgres::tokio_postgres;
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, PostgresQueryBuilder, Table};

pub async fn migrate(db: &Pool) -> Result<(), mobc::Error<tokio_postgres::Error>> {
    let con = db.get().await?;
    migrate_users(&con).await?;
    migrate_tweets(&con).await?;
    Ok(())
}

async fn migrate_tweets(db: &Connection) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .table(Tweets::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Tweets::Id)
                .big_integer()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Tweets::Text).text().not_null())
        .col(ColumnDef::new(Tweets::Timestamp).integer().not_null())
        .col(ColumnDef::new(Tweets::UserId).text().not_null())
        .col(ColumnDef::new(Tweets::Signature).text().not_null())
        .col(ColumnDef::new(Tweets::Hash).text())
        .col(ColumnDef::new(Tweets::PreviousId).big_integer())
        .foreign_key(
            ForeignKey::create()
                .from(Tweets::Table, Tweets::UserId)
                .to(Users::Table, Users::PublicKey)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .build(PostgresQueryBuilder);

    db.batch_execute(&sql).await?;
    Ok(())
}

async fn migrate_users(db: &Connection) -> Result<(), tokio_postgres::Error> {
    let sql = Table::create()
        .table(Users::Table)
        .if_not_exists()
        .col(ColumnDef::new(Users::PublicKey).text().primary_key())
        .col(
            ColumnDef::new(Users::Username)
                .char()
                .char_len(50)
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(Users::ImageURL).char().char_len(100))
        .build(PostgresQueryBuilder);

    db.batch_execute(&sql).await?;
    Ok(())
}
