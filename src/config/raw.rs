use crate::config::Config;
use serde::Deserialize;

use crate::config::db::DB;
use crate::config::server::Server;

use super::logger::Logger;

#[derive(Deserialize)]
pub(super) struct Raw {
    pub server: Server,
    pub db: DB,
    pub logger: Logger,
}

impl Into<Config> for Raw {
    fn into(self) -> Config {
        self.logger.init();

        Config {
            server: self.server.into(),
            db: self.db.into(),
        }
    }
}
