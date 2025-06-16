use lazy_supplements_core::{
    global::generate_uuid,
    cache::entity::PeerModel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub id: Uuid,
    pub content: ResponseContent,
}

impl From<ResponseContent> for Response {
    fn from(c: ResponseContent) -> Self {
        Self{
            id: generate_uuid(),
            content: c
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ResponseContent {
    Pong,
    ListPeers(Vec<PeerModel>)
}