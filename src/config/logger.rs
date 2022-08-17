use log::LevelFilter;
use serde::Deserialize;
use simplelog::{ColorChoice, TerminalMode};
use std::str::FromStr;
use thiserror::Error;

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
    pub fn init(self) -> Result<(), Error> {
        simplelog::TermLogger::init(
            self.parse_level().map_err(Error::ParseError)?,
            simplelog::Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        )
        .map_err(Error::InitError)
    }

    fn parse_level(self) -> Result<LevelFilter, log::ParseLevelError> {
        LevelFilter::from_str(self.level.to_uppercase().as_str())
    }
}
