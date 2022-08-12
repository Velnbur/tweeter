use crate::config::Config;
use serde::Deserialize;

use super::{db::DB, logger::Logger, server::Server, storage::Storage};

#[derive(Deserialize)]
pub(super) struct Raw {
    pub server: Server,
    pub db: DB,
    pub logger: Logger,
    pub storage: Storage,
}

impl Into<Config> for Raw {
    fn into(self) -> Config {
        self.logger.init();

        Config {
            server: self.server.into(),
            db: self.db.into(),
            storage: self.storage.into(),
        }
    }
}
