use iroh::NodeId;

/// Request of node authentication.
#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    sender_id: NodeId,
    sender_info: String,
}
