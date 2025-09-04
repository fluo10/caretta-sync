use iroh::endpoint::RemoteInfo;

use crate::{error::Error, proto::{DirectAddrInfoMessage, RemoteInfoMessage}};

impl TryFrom<RemoteInfo> for RemoteInfoMessage {
    type Error = Error;
    fn try_from(value: RemoteInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            node_id: Vec::from(value.node_id.as_bytes()),
            relay_url: value.relay_url.map_or(String::from(""), |x| x.relay_url.to_string()),
            addrs: value.addrs.into_iter()
                .map(|x| DirectAddrInfoMessage::try_from(x))
                .collect::<Result<Vec<DirectAddrInfoMessage>,Error>>()?,
            conn_type: value.conn_type.to_string(),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_used: value.last_used.map(|x| x.try_into()).transpose()?,
        })
    }
}