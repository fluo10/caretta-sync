use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct ListTrustedNodeRequest;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListTrustedNodeResponse {
    node: Vec<crate::data::entity::TrustedNodeModel>
}