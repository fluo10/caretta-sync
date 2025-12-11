use iroh::{
    Endpoint, PublicKey, SecretKey,
    discovery::{
        dns::DnsDiscovery,
        mdns::{DiscoveryEvent, MdnsDiscovery},
    },
    protocol::Router,
};
use redb::Database;
use serde::{Deserialize, Serialize};

use crate::util::{Emptiable, Mergeable};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2pConfig {
    pub enabled: bool,
    pub secret_key: SecretKey,
    pub enable_mdns: bool,
    pub enable_n0: bool,
}

impl P2pConfig {
    #[cfg(feature = "service")]
    pub async fn to_iroh_router(
        &self,
        app_name: &'static str,
    ) -> Result<Option<Router>, iroh::endpoint::BindError> {
        if self.enabled {
            let mut endpoint = iroh::endpoint::Builder::empty(iroh::RelayMode::Disabled)
                .secret_key(self.secret_key.clone());
            if self.enable_n0 {
                endpoint = endpoint.discovery(DnsDiscovery::n0_dns());
            }
            if self.enable_mdns {
                let mdns = MdnsDiscovery::builder().service_name(app_name);
                endpoint = endpoint.discovery(mdns);
            }
            let ep = endpoint.bind().await?;
            Ok(Some(
                Router::builder(ep)
                    .accept(iroh_ping::ALPN, iroh_ping::Ping::new())
                    .spawn(),
            ))
        } else {
            Ok(None)
        }
    }
}

impl PartialEq for P2pConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.enabled == other.enabled)
         || (self.enable_mdns == other.enable_mdns)
          || (self.enable_n0 == other.enable_n0) 
          || (self.secret_key.to_bytes() == other.secret_key.to_bytes())
    }
}

#[cfg(feature = "service")]
impl redb::Value for P2pConfig {
    type SelfType<'a> = P2pConfig;

    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a {
        ciborium::from_reader(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b 
        {
        let mut buf = Vec::new();
        ciborium::into_writer(value, &mut buf).unwrap();
        buf        
    }

    fn type_name() -> redb::TypeName {
        use redb::TypeName;

        TypeName::new(stringify!(P2pConfig))
    }
}
