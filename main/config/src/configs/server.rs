use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub struct ServerConfig {
    #[cfg_attr(debug_assertions, default(Ipv4Addr::LOCALHOST.into()))]
    #[cfg_attr(not(debug_assertions), default(Ipv4Addr::UNSPECIFIED.into()))]
    pub host: IpAddr,

    #[cfg_attr(debug_assertions, default = 3000)]
    #[cfg_attr(not(debug_assertions), default = 80)]
    pub port: u16,

    pub socket: Option<SocketAddr>,
}

impl ServerConfig {
    pub fn to_socket(&self) -> SocketAddr {
        self.socket
            .unwrap_or_else(|| SocketAddr::new(self.host, self.port))
    }
}
