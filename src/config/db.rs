use std::{str::FromStr, time::Duration};

use crate::db::Pool;
use mobc_postgres::{
    tokio_postgres::{Config, Error, NoTls},
    PgConnectionManager,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct DB {
    pub url: String,
}

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

impl DB {
    fn create_pool(url: String) -> Result<Pool, mobc::Error<Error>> {
        let config = Config::from_str(url.as_str())?;

        let manager = PgConnectionManager::new(config, NoTls);
        Ok(Pool::builder()
            .max_open(DB_POOL_MAX_OPEN)
            .max_idle(DB_POOL_MAX_IDLE)
            .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
            .build(manager))
    }
}

impl Into<Pool> for DB {
    fn into(self) -> Pool {
        Self::create_pool(self.url).expect("Failed to create pool")
    }
}
