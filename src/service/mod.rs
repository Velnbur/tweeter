use crate::db;

mod handlers;
mod routing;
mod schemas;


pub async fn run(db: db::Pool) {
    warp::serve(routing::route(db))
        .run(([127, 0, 0, 1], 8080))
        .await;
}