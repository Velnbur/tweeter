use std::str::FromStr;

use log::LevelFilter;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Logger {
    pub level: String,
}

use simplelog::{ColorChoice, TerminalMode};

impl Logger {
    pub fn init(self) {
        simplelog::TermLogger::init(
            self.parse_level(),
            simplelog::Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        )
        .expect("Failed to init logger");
    }

    fn parse_level(self) -> LevelFilter {
        LevelFilter::from_str(self.level.to_uppercase().as_str())
            .expect("Invalid level in logger config")
    }
}
