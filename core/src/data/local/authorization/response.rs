use iroh::NodeId;

/// Response of node authentication.
#[derive(Debug, Clone)]
pub struct AuthorizationResponse {
    sender_id: NodeId,
    passcode: String,
}




