use serde::Deserialize;
use std::{net::SocketAddrV4, str::FromStr};

#[derive(Deserialize)]
pub(super) struct Server {
    pub addr: String,
}

impl Server {
    pub fn parse(self) -> Result<SocketAddrV4, std::net::AddrParseError> {
        SocketAddrV4::from_str(&self.addr)
    }
}
