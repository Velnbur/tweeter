extern crate core;

mod service;
mod records;
pub mod db;

#[tokio::main]
async fn main() {
    let pool = db::create_pool("postgresql://test:test@localhost:5432/test".to_string())
        .expect("Failed to init db");

    let con = db::get_con(&pool)
        .await
        .expect("Failed to get connection");
    records::migrations::migrate(con)
        .await
        .expect("Failed to migrate");

    service::run(pool).await;
}
