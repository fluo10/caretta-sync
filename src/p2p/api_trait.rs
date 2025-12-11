use crate::types::{EndpointPublicKey, InvitationToken, NamespaceSecretKey};

/// A trait for 
#[async_trait::async_trait]
pub trait P2pApiTrait{
    type Error;
    /// Get device information
    async fn join(&self, token: InvitationToken) -> Result<(NamespaceSecretKey, Vec<EndpointPublicKey>), Self::Error>;
    async fn ping(&self, id: EndpointPublicKey) -> Result<(), Self::Error>;
}