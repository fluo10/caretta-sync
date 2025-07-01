use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListDeviceResponse {
    node: Vec<crate::data::entity::TrustedNode>
}