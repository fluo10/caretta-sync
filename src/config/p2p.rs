use iroh::{
    Endpoint, PublicKey, SecretKey,
    discovery::{
        dns::DnsDiscovery,
        mdns::{DiscoveryEvent, MdnsDiscovery},
    },
    protocol::Router,
};
#[cfg(feature = "server")]
use iroh_blobs::BlobsProtocol;
#[cfg(feature = "server")]
use iroh_docs::protocol::Docs;
#[cfg(feature = "server")]
use iroh_gossip::Gossip;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::config::StorageConfig;
use crate::{
    types::EndpointSecretKey,
    util::{Emptiable, Mergeable},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2pConfig {
    pub secret_key: EndpointSecretKey,
    pub enable_mdns: bool,
    pub enable_n0: bool,
}

impl P2pConfig {
    #[cfg(feature = "server")]
    pub(crate) async fn spawn_iroh_endpoint(
        &self,
        app_name: &'static str,
    ) -> Result<Endpoint, iroh::endpoint::BindError> {
        let mut endpoint = iroh::endpoint::Builder::empty(iroh::RelayMode::Disabled)
            .secret_key(self.secret_key.clone().into());
        if self.enable_n0 {
            endpoint = endpoint.discovery(DnsDiscovery::n0_dns());
        }
        if self.enable_mdns {
            let mdns = MdnsDiscovery::builder().service_name(app_name);
            endpoint = endpoint.discovery(mdns);
        }
        endpoint.bind().await
    }
}

impl PartialEq for P2pConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.enable_mdns == other.enable_mdns)
            || (self.enable_n0 == other.enable_n0)
            || (self.secret_key.to_bytes() == other.secret_key.to_bytes())
    }
}
