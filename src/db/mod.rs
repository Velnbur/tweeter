use mobc_postgres::{PgConnectionManager, tokio_postgres};
use mobc_postgres::tokio_postgres::NoTls;
use tokio_postgres::{Config, Error};
use std::str::FromStr;
use std::time::Duration;

pub mod errors;

pub type Pool = mobc::Pool<PgConnectionManager<NoTls>>;
pub type Connection = mobc::Connection<PgConnectionManager<NoTls>>;

pub async fn get_con(pool: &Pool) -> Result<Connection, errors::Error> {
    pool.get().await.map_err(errors::Error::PoolError)
}

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool(url: String) -> Result<Pool, mobc::Error<Error>> {
    let config = Config::from_str(url.as_str())?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}
