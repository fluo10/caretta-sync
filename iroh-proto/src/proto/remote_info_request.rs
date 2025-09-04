use iroh::NodeId;

use crate::proto::RemoteInfoRequest;

impl From<NodeId> for RemoteInfoRequest {
    fn from(value: NodeId) -> Self {
        Self {
            node_id : Vec::from(value.as_bytes())
        }
    }
}

impl TryFrom<RemoteInfoRequest> for NodeId {
    type Error = crate::error::Error;
    fn try_from(value: RemoteInfoRequest) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.node_id[0..32].try_into()?;
        Ok(NodeId::from_bytes(&slice)?)
    }
}