use lazy_supplements_core::global::generate_uuid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: Uuid,
    pub content: RequestContent,
}

impl From<RequestContent> for Request {
    fn from(c: RequestContent) -> Self {
        Self{
            id: generate_uuid(),
            content: c
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestContent {
    Ping,
    ListPeers,
}