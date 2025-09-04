use crate::{ proto::{RemoteInfoMessage, RemoteInfoResponse}};

impl From<RemoteInfoMessage> for RemoteInfoResponse {
    fn from(value: RemoteInfoMessage) -> Self {
        Self {
            remote_info: Some(value)
        }
    }
}
impl From<Option<RemoteInfoMessage>> for RemoteInfoResponse {
    fn from(value: Option<RemoteInfoMessage>) -> Self {
        Self{
            remote_info: value,
        }
    }
}