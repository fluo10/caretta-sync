use crate::proto::{
    ProtoDeserializeError,
};

tonic::include_proto!("caretta_sync.types.iroh");

impl From<iroh_docs::NamespaceSecret> for DocNamespaceSecret {
    fn from(value: iroh_docs::NamespaceSecret) -> Self {
        Self{
            value: Vec::from(value.to_bytes())
        }
    }
}

impl TryFrom<DocNamespaceSecret> for iroh_docs::NamespaceSecret {
    type Error = ProtoDeserializeError;
    fn try_from(value: DocNamespaceSecret) -> Result<Self, Self::Error> {
        let slice: [u8;32] = value.value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice))
    }
}


impl From<iroh::PublicKey> for EndpointId {
    fn from(value: iroh::PublicKey) -> Self {
        Self {
            value: Vec::from(value.as_bytes()),
        }
    }
}

impl TryFrom<EndpointId> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: EndpointId) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.value[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}

impl From<iroh_tickets::endpoint::EndpointTicket> for EndpointTicket {
    fn from(value: iroh_tickets::endpoint::EndpointTicket) -> Self {
        Self {
            value: value.to_string()
        }
    }
}

impl TryFrom<EndpointTicket> for iroh_tickets::endpoint::EndpointTicket {
    type Error = ProtoDeserializeError;
    fn try_from(value: EndpointTicket) -> Result<Self, Self::Error> {
        Ok(value.value.parse()?)
    }
}