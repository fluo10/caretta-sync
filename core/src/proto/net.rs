tonic::include_proto!("caretta_sync.net");

use crate::proto::error::{ProtoDeserializeError, ProtoSerializeError};

type Ipv4AddrMessage = Ipv4Addr;
type Ipv6AddrMessage = Ipv6Addr;
type SocketAddrMessage = SocketAddr;
type SocketAddrV4Message = SocketAddrV4;
type SocketAddrV6Message = SocketAddrV6;

impl From<std::net::SocketAddr> for SocketAddrMessage {
    fn from(value: std::net::SocketAddr) -> Self {
        Self {
            socket_addr_value: Some(match value {
                std::net::SocketAddr::V4(x) => {
                    socket_addr::SocketAddrValue::V4(SocketAddrV4Message::from(x))
                }
                std::net::SocketAddr::V6(x) => {
                    socket_addr::SocketAddrValue::V6(SocketAddrV6Message::from(x))
                }
            }),
        }
    }
}

impl TryFrom<SocketAddrMessage> for std::net::SocketAddr {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrMessage) -> Result<Self, Self::Error> {
        Ok(
            match value
                .socket_addr_value
                .ok_or(Self::Error::MissingField("SocketAddr.socket_addr"))?
            {
                socket_addr::SocketAddrValue::V4(x) => std::net::SocketAddr::V4(x.try_into()?),
                socket_addr::SocketAddrValue::V6(x) => std::net::SocketAddr::V6(x.try_into()?),
            },
        )
    }
}

impl From<std::net::SocketAddrV4> for SocketAddrV4Message {
    fn from(value: std::net::SocketAddrV4) -> Self {
        Self {
            ip: Some(value.ip().clone().into()),
            port: value.port().into(),
        }
    }
}

impl TryFrom<SocketAddrV4Message> for std::net::SocketAddrV4 {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrV4Message) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value
                .ip
                .ok_or(ProtoDeserializeError::MissingField("SocketAddrV4.ip"))?
                .into(),
            value.port.try_into()?,
        ))
    }
}

impl From<std::net::Ipv4Addr> for Ipv4AddrMessage {
    fn from(value: std::net::Ipv4Addr) -> Self {
        Self {
            bits: value.to_bits(),
        }
    }
}
impl From<Ipv4AddrMessage> for std::net::Ipv4Addr {
    fn from(value: Ipv4AddrMessage) -> Self {
        Self::from_bits(value.bits)
    }
}

impl From<std::net::SocketAddrV6> for SocketAddrV6Message {
    fn from(value: std::net::SocketAddrV6) -> Self {
        Self {
            ip: Some(value.ip().clone().into()),
            port: value.port().into(),
        }
    }
}

impl TryFrom<SocketAddrV6Message> for std::net::SocketAddrV6 {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrV6Message) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value
                .ip
                .ok_or(ProtoDeserializeError::MissingField("SocketAddrV6.ip"))?
                .into(),
            value.port.try_into()?,
            0,
            0,
        ))
    }
}

impl From<std::net::Ipv6Addr> for Ipv6AddrMessage {
    fn from(value: std::net::Ipv6Addr) -> Self {
        let bits = value.to_bits();

        Self {
            high_bits: (bits >> 64) as u64,
            low_bits: bits as u64,
        }
    }
}
impl From<Ipv6AddrMessage> for std::net::Ipv6Addr {
    fn from(value: Ipv6AddrMessage) -> Self {
        Self::from_bits(((value.high_bits as u128) << 64) + (value.low_bits as u128))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{self, Ipv4Addr},
        u8, u16,
    };

    use rand::random;

    use super::*;

    fn validate_socket_addr_conversion(
        socket_addr: net::SocketAddr,
    ) -> Result<bool, ProtoDeserializeError> {
        let message = SocketAddrMessage::from(socket_addr);
        Ok(socket_addr == message.try_into()?)
    }

    #[test]
    fn socket_addr_conversion_ipv4_min() {
        assert!(
            validate_socket_addr_conversion(net::SocketAddr::new(
                net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)),
                u16::MIN
            ))
            .unwrap()
        );
    }
    #[test]
    fn socket_addr_conversion_ipv4_max() {
        assert!(
            validate_socket_addr_conversion(net::SocketAddr::new(
                net::IpAddr::V4(net::Ipv4Addr::new(u8::MAX, u8::MAX, u8::MAX, u8::MAX)),
                u16::MAX
            ))
            .unwrap()
        );
    }
    #[test]
    fn socket_addr_conversion_ipv4_random() {
        for _ in 0..10 {
            assert!(
                validate_socket_addr_conversion(net::SocketAddr::new(
                    net::IpAddr::V4(net::Ipv4Addr::new(random(), random(), random(), random())),
                    random()
                ))
                .unwrap()
            )
        }
    }
    #[test]
    fn socket_addr_conversion_ipv6_min() {
        assert!(
            validate_socket_addr_conversion(net::SocketAddr::new(
                net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                u16::MIN
            ))
            .unwrap()
        );
    }
    #[test]
    fn socket_addr_conversion_ipv6_max() {
        assert!(
            validate_socket_addr_conversion(net::SocketAddr::new(
                net::IpAddr::V6(net::Ipv6Addr::new(
                    u16::MAX,
                    u16::MAX,
                    u16::MAX,
                    u16::MAX,
                    u16::MAX,
                    u16::MAX,
                    u16::MAX,
                    u16::MAX
                )),
                u16::MAX
            ))
            .unwrap()
        );
    }
}
