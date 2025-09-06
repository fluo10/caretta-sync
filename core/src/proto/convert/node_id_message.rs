use iroh::NodeId;

use crate::proto::{error::{ProtoDeserializeError, ProtoSerializeError}, NodeIdMessage};

impl From<NodeId> for NodeIdMessage {
    fn from(value: NodeId) -> Self {
        NodeIdMessage { node_id: Vec::from(value.as_bytes()) }
    }
} 

impl TryFrom<NodeIdMessage> for NodeId {
    type Error = ProtoDeserializeError;
    fn try_from(value: NodeIdMessage) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.node_id[0..32].try_into()?;
        Ok(NodeId::from_bytes(&slice)?)
    }
}