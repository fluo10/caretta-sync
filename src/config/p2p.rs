use iroh::{
    Endpoint, PublicKey, SecretKey,
    discovery::{
        dns::DnsDiscovery,
        mdns::{DiscoveryEvent, MdnsDiscovery},
    },
    protocol::Router,
};
#[cfg(feature = "engine")]
use iroh_blobs::BlobsProtocol;
#[cfg(feature = "engine")]
use iroh_docs::protocol::Docs;
#[cfg(feature = "engine")]
use iroh_gossip::Gossip;
use serde::{Deserialize, Serialize};

#[cfg(feature = "engine")]
use crate::config::StorageConfig;
use crate::{types::EndpointSecretKey, util::{Emptiable, Mergeable}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct P2pConfig {
    pub secret_key: EndpointSecretKey,
    pub enable_mdns: bool,
    pub enable_n0: bool,
}

impl P2pConfig {
    #[cfg(feature = "engine")]
    pub(crate) async fn spawn_iroh_endpoint(&self) -> Result<Endpoint, iroh::endpoint::BindError> {
        let mut endpoint = iroh::endpoint::Builder::empty(iroh::RelayMode::Disabled)
            .secret_key(self.secret_key.clone().into());
        if self.enable_n0 {
            endpoint = endpoint.discovery(DnsDiscovery::n0_dns());
        }
        if self.enable_mdns {
            use crate::APP_NAME;

            let mdns = MdnsDiscovery::builder().service_name(APP_NAME);
            endpoint = endpoint.discovery(mdns);
        }
        endpoint.bind().await
    }
    #[cfg(feature = "engine")]
    pub async fn spawn_iroh_protocols(
        &self,
        storage_config: &StorageConfig
    ) -> Result<(Endpoint, BlobsProtocol, Docs, Gossip), iroh::endpoint::BindError> {
        use iroh_blobs::BlobsProtocol;
        use iroh_gossip::Gossip;
        let endpoint = self.spawn_iroh_endpoint().await.unwrap();

        let iroh_dir = storage_config.to_iroh_path();
        let blobs = iroh_blobs::store::fs::FsStore::load(&iroh_dir.join("blobs")).await.unwrap();
        let gossip = Gossip::builder().spawn(endpoint.clone());
        let docs = Docs::persistent(iroh_dir.join("docs")).spawn(endpoint.clone(), blobs.clone().into(), gossip.clone()).await.unwrap();

        Ok((
            endpoint, 
            BlobsProtocol::new(&blobs, None),
            docs,
            gossip,
        ))
    }
}

impl PartialEq for P2pConfig {
    fn eq(&self, other: &Self) -> bool {
         (self.enable_mdns == other.enable_mdns)
          || (self.enable_n0 == other.enable_n0) 
          || (self.secret_key.to_bytes() == other.secret_key.to_bytes())
    }
}
