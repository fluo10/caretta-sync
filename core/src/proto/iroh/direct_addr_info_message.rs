use iroh::endpoint::DirectAddrInfo;
use prost_types::DurationError;

use crate::proto::iroh::{DirectAddrInfoMessage, SourceMessage};

impl TryFrom<DirectAddrInfo> for DirectAddrInfoMessage {
    type Error = DurationError;
    fn try_from(value: DirectAddrInfo) -> Result<Self, Self::Error> {
        Ok(DirectAddrInfoMessage {
            addr: value.addr.to_string(),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_control: value.last_control.map(|x| super::LastControlMessage::try_from(x)).transpose()?,
            last_payload: value.last_payload.map(|x| x.try_into()).transpose()?,
            last_alive: value.last_alive.map(|x| x.try_into()).transpose()?,
            sources: value.sources.into_iter().map(|x| SourceMessage::try_from(x)).collect::<Result<Vec<SourceMessage>, DurationError>>()?
        })
    }
}