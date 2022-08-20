use std::net::{Ipv4Addr, SocketAddrV4};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Server {
    pub port: u16,
}

impl Into<SocketAddrV4> for Server {
    fn into(self) -> SocketAddrV4 {
        SocketAddrV4::new(
            Ipv4Addr::new(127, 0, 0, 1),
            self.port
        )
    }
}