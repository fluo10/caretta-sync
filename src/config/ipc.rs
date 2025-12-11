use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
#[cfg(unix)]
use std::path::PathBuf;

#[cfg(unix)]
static DEFAULT_PORT: u16 = 54321;

#[derive(Clone, Debug)]
pub struct IpcConfig {
    pub endpoint: SocketAddr
}

impl IpcConfig {
    pub fn default(app_name: &'static str) -> Self{
        
        Self{
            endpoint: SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, DEFAULT_PORT, 0, 0))
        }
    
    }
}