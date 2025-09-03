use iroh::NodeId;

use crate::proto::iroh::RemoteInfoRequest;

impl From<NodeId> for RemoteInfoRequest {
    fn from(value: NodeId) -> Self {
        Self {
            node_id : value.to_string()
        }
    }
}