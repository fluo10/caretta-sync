use std::time::Duration;


use crate::{error::Error, proto::{error::ProtoSerializeError, generated::remote_node}};


impl TryFrom<(iroh::endpoint::Source, Duration)> for remote_node::RemoteNodeSource {
    type Error = ProtoSerializeError;
    fn try_from(src: (iroh::endpoint::Source, Duration)) -> Result<Self, Self::Error> {
        let (source, duration )= src;
        Ok(Self {
            source: source.to_string(),
            duration: Some(duration.try_into()?),
        })
    }
} 