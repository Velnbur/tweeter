use log::LevelFilter;
use serde::Deserialize;
use simplelog::{ColorChoice, TerminalMode};
use std::str::FromStr;
use thiserror::Error;

use crate::Parseable;

#[derive(Deserialize)]
pub struct Logger {
    pub level: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to init logger: {0}")]
    InitError(#[from] log::SetLoggerError),
    #[error("failed to parse level: {0}")]
    ParseError(#[from] log::ParseLevelError),
}

impl Logger {
    fn parse_level(self) -> Result<LevelFilter, Error> {
        let level = self.level.to_uppercase();

        LevelFilter::from_str(level.as_str()).map_err(Error::ParseError)
    }
}

#[async_trait::async_trait]
impl Parseable for Logger {
    type Error = Error;
    type Value = ();

    async fn parse(self) -> Result<Self::Value, Self::Error> {
        simplelog::TermLogger::init(
            self.parse_level()?,
            simplelog::Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        )
        .map_err(Error::InitError)
    }
}
