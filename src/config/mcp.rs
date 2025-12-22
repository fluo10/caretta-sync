use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};

/// Generate default port from hash of app_name and username
fn gen_default_port(app_name: &'static str) -> u16 {
    let mut output = [0; 2];
    let mut hasher = blake3::Hasher::new();
    hasher.update(app_name.as_bytes());
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut output);
    let i = u16::from_be_bytes(output);
    i & 0b1100_0000_0000_0000
}

#[derive(Clone, Debug)]
pub struct McpConfig {
    pub endpoint_url: String,
    pub listen_addr: SocketAddr,
    pub access_token: Option<String>,
}

impl McpConfig {
    pub fn default(app_name: &'static str) -> Self {
        let default_port = gen_default_port(app_name);
        let socket_addr =
            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, default_port, 0, 0));

        Self {
            endpoint_url: format!("http://{}", &socket_addr),
            listen_addr: socket_addr,
            access_token: None,
        }
    }

    #[cfg(feature = "desktop-server")]
    pub async fn bind_tcp_listener(&self) -> tokio::net::TcpListener {
        tokio::net::TcpListener::bind(self.listen_addr)
            .await
            .unwrap()
    }
}
