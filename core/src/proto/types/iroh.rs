

use crate::proto::{
    ProtoDeserializeError, ProtoSerializeError, types::net::SocketAddr,
};

tonic::include_proto!("caretta_sync.types.iroh");

impl From<iroh::endpoint::ConnectionType> for ConnectionType {
    fn from(value: iroh::endpoint::ConnectionType) -> Self {
        use connection_type::*;
        Self {
            value: Some(match value {
                iroh::endpoint::ConnectionType::Direct(socket_addr) => {
                    connection_type::Value::Direct(connection_type::Direct {
                        addr: Some(SocketAddr::from(socket_addr)),
                    })
                }
                iroh::endpoint::ConnectionType::Relay(relay_url) => {
                    connection_type::Value::Relay(connection_type::Relay {
                        url: Some(super::url::Url::from((*relay_url).clone())),
                    })
                }
                iroh::endpoint::ConnectionType::Mixed(socket_addr, relay_url) => {
                    connection_type::Value::Mixed(connection_type::Mixed {
                        addr: Some(SocketAddr::from(socket_addr)),
                        url: Some(super::url::Url::from((*relay_url).clone())),
                    })
                }
                iroh::endpoint::ConnectionType::None => Value::None(None {}),
            }),
        }
    }
}

impl From<iroh::endpoint::ControlMsg> for ControlMsg {
    fn from(value: iroh::endpoint::ControlMsg) -> Self {
        use control_msg::*;
        Self {
            value: Some(match value {
                iroh::endpoint::ControlMsg::Ping => Value::Ping(Ping {}),
                iroh::endpoint::ControlMsg::Pong => Value::Pong(Pong {}),
                iroh::endpoint::ControlMsg::CallMeMaybe => {
                    Value::CallMeMaybe(CallMeMayBe {})
                }
            }),
        }
    }
}

impl TryFrom<iroh::endpoint::DirectAddrInfo> for DirectAddrInfo {
    type Error = ProtoSerializeError;
    fn try_from(value: iroh::endpoint::DirectAddrInfo) -> Result<Self, Self::Error> {
        use direct_addr_info::*;
        let last_control: Option<DurationControlMsg> =
            if let Some((duration, control_msg)) = value.last_control {
                Some(DurationControlMsg {
                    control_msg: Some(control_msg.into()),
                    duration: Some(duration.try_into()?),
                })
            } else {
                None
            };
        Ok(Self {
            addr: Some(value.addr.into()),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_control,
            last_payload: value.last_payload.map(|x| x.try_into()).transpose()?,
            last_alive: value.last_alive.map(|x| x.try_into()).transpose()?,
            sources: value
                .sources
                .into_iter()
                .map(|(s, d)| {
                    Ok::<SourceDuration, ProtoSerializeError>(SourceDuration {
                        source: Some(s.into()),
                        duration: Some(d.try_into()?),
                    })
                })
                .collect::<Result<Vec<SourceDuration>, ProtoSerializeError>>()?,
        })
    }
}

impl From<iroh::PublicKey> for PublicKey {
    fn from(value: iroh::PublicKey) -> Self {
        Self {
            value: Vec::from(value.as_bytes()),
        }
    }
}

impl TryFrom<PublicKey> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: PublicKey) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.value[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}

impl TryFrom<iroh::endpoint::RemoteInfo> for RemoteInfo {
    type Error = ProtoSerializeError;
    fn try_from(value: iroh::endpoint::RemoteInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            node_id: Some(value.node_id.into()),
            relay_url: value
                .relay_url
                .map(|x| {
                    Ok::<RelayUrlInfo, ProtoSerializeError>(RelayUrlInfo {
                        relay_url: Some((*x.relay_url).clone().into()),
                        last_alive: x.last_alive.map(|x| x.try_into()).transpose()?,
                        latency: x.latency.map(|x| x.try_into()).transpose()?,
                    })
                })
                .transpose()?,
            addrs: value
                .addrs
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<DirectAddrInfo>, ProtoSerializeError>>()?,
            conn_type: Some(value.conn_type.into()),
            latency: value.latency.map(|x| x.try_into()).transpose()?,
            last_used: value.last_used.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl From<iroh::endpoint::Source> for Source {
    fn from(value: iroh::endpoint::Source) -> Self {
        use source::*;
        Self {
            value: Some(match value {
                iroh::endpoint::Source::Saved => Value::Saved(Saved {}),
                iroh::endpoint::Source::Udp => Value::Udp(Udp {}),
                iroh::endpoint::Source::Relay => Value::Relay(Relay {}),
                iroh::endpoint::Source::App => Value::App(App {}),
                iroh::endpoint::Source::Discovery { name } => {
                    Value::Discovery(Discovery { value: name })
                }
                iroh::endpoint::Source::NamedApp { name } => {
                    Value::NamedApp(NamedApp { value: name })
                }
            }),
        }
    }
}
