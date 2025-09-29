pub use crate::proto::generated::common;

use crate::proto::{error::{ProtoDeserializeError, ProtoSerializeError}};

impl From<iroh::PublicKey> for common::PublicKey {
    fn from(value: iroh::PublicKey) -> Self {
        common::PublicKey{ key: Vec::from(value.as_bytes()) }
    }
} 

impl TryFrom<common::PublicKey> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: common::PublicKey) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.key[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}

impl From<uuid::Uuid> for common::Uuid {
    fn from(value: uuid::Uuid) -> Self {
        let (first_half, second_half) = value.as_u64_pair();
        Self {
            high_bits: first_half,
            low_bits: second_half
        }
    }
}

impl From<common::Uuid> for uuid::Uuid {
    fn from(value: common::Uuid) -> Self {
        uuid::Uuid::from_u64_pair(value.high_bits, value.low_bits)
    }
}



impl From<url::Url> for common::Url {
    fn from(value: url::Url) -> Self {
        todo!()
    }
}

impl TryFrom<common::Url> for url::Url {
    type Error = ProtoDeserializeError;
    fn try_from(value: common::Url) -> Result<Self, Self::Error> {
        todo!()
    }
} 

impl From<std::net::SocketAddr> for common::SocketAddr {
    fn from(value: std::net::SocketAddr) -> Self {
        Self{
            socket_addr: Some(match value {
            std::net::SocketAddr::V4(x) => common::socket_addr::SocketAddr::V4(common::SocketAddrV4::from(x)),
            std::net::SocketAddr::V6(x) => common::socket_addr::SocketAddr::V6(common::SocketAddrV6::from(x)),
        })}
    }
}

impl TryFrom<common::SocketAddr> for std::net::SocketAddr {
    type Error = ProtoDeserializeError;
    fn try_from(value: common::SocketAddr) -> Result<Self, Self::Error> {
        Ok(match value.socket_addr.ok_or(Self::Error::MissingField("SocketAddr.socket_addr"))? {
            common::socket_addr::SocketAddr::V4(x) => std::net::SocketAddr::V4(x.try_into()?),
            common::socket_addr::SocketAddr::V6(x) => std::net::SocketAddr::V6(x.try_into()?),
        })
    }
}

impl From<std::net::SocketAddrV4> for common::SocketAddrV4 {
    fn from(value: std::net::SocketAddrV4) -> Self {
        Self {
            ip : Some(value.ip().clone().into()),
            port: value.port().into(),
        }
    }
}

impl TryFrom<common::SocketAddrV4> for std::net::SocketAddrV4 {
    type Error = ProtoDeserializeError;
    fn try_from(value: common::SocketAddrV4) -> Result<Self, Self::Error> {
        Ok(Self::new(value.ip.ok_or(ProtoDeserializeError::MissingField("SocketAddrV4.ip"))?.into(), value.port.try_into()?))
    }
}

impl From<std::net::Ipv4Addr> for common::Ipv4Addr {
    fn from(value: std::net::Ipv4Addr) -> Self {
        Self{
            bits: value.to_bits()
        }
    }
}
impl From<common::Ipv4Addr> for std::net::Ipv4Addr {
    fn from(value: common::Ipv4Addr) -> Self{
        Self::from_bits(value.bits)
    }
}

impl From<std::net::SocketAddrV6> for common::SocketAddrV6 {
    fn from(value: std::net::SocketAddrV6) -> Self {
        Self{
            ip: Some(value.ip().clone().into()),
            port: value.port().into()
        }
    }
}

impl TryFrom<common::SocketAddrV6> for std::net::SocketAddrV6 {
    type Error = ProtoDeserializeError;
    fn try_from(value: common::SocketAddrV6) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.ip.ok_or(ProtoDeserializeError::MissingField("SocketAddrV6.ip"))?.into(),
            value.port.try_into()?,
            0,
            0
        ))   
    }
}

impl From<std::net::Ipv6Addr> for common::Ipv6Addr {
    fn from(value: std::net::Ipv6Addr) -> Self {
        let bits = value.to_bits();

        Self{
            high_bits: (bits >> 64) as u64,
            low_bits: bits as u64,
        }
    }
}
impl From<common::Ipv6Addr> for std::net::Ipv6Addr{
    
    fn from(value: common::Ipv6Addr) -> Self {
        Self::from_bits(
            ((value.high_bits as u128) << 64) + (value.low_bits as u128) 
        )        
    }
}

#[cfg(test)]
mod tests {
    use std::{net::{self, Ipv4Addr}, u16};

    use super::*;
    fn validate_uuid_conversion(uuid: uuid::Uuid) -> bool{
        let message = common::Uuid::from(uuid);
        uuid == uuid::Uuid::from(message)
    }

    #[test]
    fn uuid_conversion() {
        assert!(validate_uuid_conversion(uuid::Uuid::nil()));
        assert!(validate_uuid_conversion(uuid::Uuid::max()));
        assert!(validate_uuid_conversion(uuid::Uuid::now_v7()));
    }

    fn validate_socket_addr_conversion(socket_addr: net::SocketAddr) -> Result<bool, ProtoDeserializeError> {
        let message = common::SocketAddr::from(socket_addr);
        Ok(socket_addr == message.try_into()?)
    }

    #[test]
    fn socket_addr_conversion() {
        assert!(validate_socket_addr_conversion(net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)),u16::MIN)).unwrap());
        assert!(validate_socket_addr_conversion(net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::BROADCAST),u16::MAX)).unwrap());
        assert!(validate_socket_addr_conversion(net::SocketAddr::new(net::IpAddr::V6(net::Ipv6Addr::new(0,0,0,0,0,0,0,0)), u16::MAX)).unwrap());
        assert!(validate_socket_addr_conversion(net::SocketAddr::new(net::IpAddr::V6(net::Ipv6Addr::new(u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX)), u16::MIN)).unwrap());


    }

}