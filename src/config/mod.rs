mod db;
mod logger;
mod raw;
mod server;

use std::fs;
use std::net::SocketAddrV4;

use crate::db::Pool;

pub struct Config {
    pub db: Pool,
    pub server: SocketAddrV4,
}

impl Config {
    pub fn from_file(filename: String) -> Self {
        let content = fs::read_to_string(filename).expect("Failed to read config file");

        let raw_config: raw::Raw = toml::from_str(&content).expect("Failed to parse config");

        raw_config.into()
    }
}
