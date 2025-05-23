use std::{collections::HashSet, net::IpAddr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    listen_ips: HashSet<IpAddr>,
    port: i32,
}
