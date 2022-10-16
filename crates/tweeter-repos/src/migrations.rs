use crate::tables::{Tweets, Users};
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, PostgresQueryBuilder, Table};
use sqlx::PgPool;

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    migrate_users(&pool).await?;
    migrate_tweets(&pool).await?;
    Ok(())
}

async fn migrate_tweets(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut con = pool.acquire().await.map_err(sqlx::Error::from)?;
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

    sqlx::query(&sql).execute(&mut con).await?;
    Ok(())
}

async fn migrate_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut con = pool.acquire().await.map_err(sqlx::Error::from)?;
    let sql = Table::create()
        .table(Users::Table)
        .if_not_exists()
        .col(ColumnDef::new(Users::PublicKey).text().primary_key())
        .col(
            ColumnDef::new(Users::Username)
                .text()
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(Users::ImageURL).text())
        .build(PostgresQueryBuilder);

    sqlx::query(&sql).execute(&mut con).await?;
    Ok(())
}
