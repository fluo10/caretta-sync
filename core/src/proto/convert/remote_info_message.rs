use iroh::endpoint::RemoteInfo;

use crate::{error::Error, proto::{error::ProtoSerializeError, DirectAddrInfoMessage, RemoteInfoMessage}};

impl TryFrom<RemoteInfo> for RemoteInfoMessage {
    type Error = ProtoSerializeError;
    fn try_from(value: RemoteInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            node_id: Some(value.node_id.into()),
            relay_url: value.relay_url.map_or(String::from(""), |x| x.relay_url.to_string()),
            addrs: value.addrs.into_iter()
                .map(|x| DirectAddrInfoMessage::try_from(x))
                .collect::<Result<Vec<DirectAddrInfoMessage>,Self::Error>>()?,
            conn_type: value.conn_type.to_string(),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_used: value.last_used.map(|x| x.try_into()).transpose()?,
        })
    }
}