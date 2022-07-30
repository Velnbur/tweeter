use serde::Deserialize;
use crate::db::{create_pool, Pool};

#[derive(Deserialize)]
pub(super) struct DB {
    pub url: String,
}

impl Into<Pool> for DB {
    fn into(self) -> Pool {
        create_pool(self.url)
            .expect("Failed to create pool")
    }
}