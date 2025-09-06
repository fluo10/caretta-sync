use iroh::NodeId;

use crate::proto::{error::ProtoDeserializeError, NodeIdMessage, RemoteInfoRequest};

impl From<NodeIdMessage> for RemoteInfoRequest {
    fn from(value: NodeIdMessage) -> Self {
        Self {
            node_id : Some(value)
        }
    }
}