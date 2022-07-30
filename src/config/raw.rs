use serde::Deserialize;
use crate::config::Config;

use crate::config::db::DB;
use crate::config::server::Server;

#[derive(Deserialize)]
pub(super) struct Raw {
    pub server: Server,
    pub db: DB,
}

impl Into<Config> for Raw {
    fn into(self) -> Config {
        Config {
            server: self.server.into(),
            db: self.db.into(),
        }
    }
}