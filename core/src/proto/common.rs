use super::*;

use crate::proto::{error::{ProtoDeserializeError, ProtoSerializeError}, socket_addr};

impl From<iroh::PublicKey> for PublicKeyMessage {
    fn from(value: iroh::PublicKey) -> Self {
        Self{ key: Vec::from(value.as_bytes()) }
    }
} 

impl TryFrom<PublicKeyMessage> for iroh::PublicKey {
    type Error = ProtoDeserializeError;
    fn try_from(value: PublicKeyMessage) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value.key[0..32].try_into()?;
        Ok(iroh::PublicKey::from_bytes(&slice)?)
    }
}

impl From<uuid::Uuid> for UuidMessage {
    fn from(value: uuid::Uuid) -> Self {
        let (first_half, second_half) = value.as_u64_pair();
        Self {
            high_bits: first_half,
            low_bits: second_half
        }
    }
}

impl From<UuidMessage> for uuid::Uuid {
    fn from(value: UuidMessage) -> Self {
        uuid::Uuid::from_u64_pair(value.high_bits, value.low_bits)
    }
}



impl From<url::Url> for UrlMessage {
    fn from(value: url::Url) -> Self {
        todo!()
    }
}

impl TryFrom<UrlMessage> for url::Url {
    type Error = ProtoDeserializeError;
    fn try_from(value: UrlMessage) -> Result<Self, Self::Error> {
        todo!()
    }
} 

impl From<std::net::SocketAddr> for SocketAddrMessage {
    fn from(value: std::net::SocketAddr) -> Self {
        Self{
            socket_addr: Some(match value {
            std::net::SocketAddr::V4(x) => socket_addr::SocketAddr::V4(SocketAddrV4Message::from(x)),
            std::net::SocketAddr::V6(x) => socket_addr::SocketAddr::V6(SocketAddrV6Message::from(x)),
        })}
    }
}

impl TryFrom<SocketAddrMessage> for std::net::SocketAddr {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrMessage) -> Result<Self, Self::Error> {
        Ok(match value.socket_addr.ok_or(Self::Error::MissingField("SocketAddr.socket_addr"))? {
            socket_addr::SocketAddr::V4(x) => std::net::SocketAddr::V4(x.try_into()?),
            socket_addr::SocketAddr::V6(x) => std::net::SocketAddr::V6(x.try_into()?),
        })
    }
}

impl From<std::net::SocketAddrV4> for SocketAddrV4Message {
    fn from(value: std::net::SocketAddrV4) -> Self {
        Self {
            ip : Some(value.ip().clone().into()),
            port: value.port().into(),
        }
    }
}

impl TryFrom<SocketAddrV4Message> for std::net::SocketAddrV4 {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrV4Message) -> Result<Self, Self::Error> {
        Ok(Self::new(value.ip.ok_or(ProtoDeserializeError::MissingField("SocketAddrV4.ip"))?.into(), value.port.try_into()?))
    }
}

impl From<std::net::Ipv4Addr> for Ipv4AddrMessage {
    fn from(value: std::net::Ipv4Addr) -> Self {
        Self{
            bits: value.to_bits()
        }
    }
}
impl From<Ipv4AddrMessage> for std::net::Ipv4Addr {
    fn from(value: Ipv4AddrMessage) -> Self{
        Self::from_bits(value.bits)
    }
}

impl From<std::net::SocketAddrV6> for SocketAddrV6Message {
    fn from(value: std::net::SocketAddrV6) -> Self {
        Self{
            ip: Some(value.ip().clone().into()),
            port: value.port().into()
        }
    }
}

impl TryFrom<SocketAddrV6Message> for std::net::SocketAddrV6 {
    type Error = ProtoDeserializeError;
    fn try_from(value: SocketAddrV6Message) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.ip.ok_or(ProtoDeserializeError::MissingField("SocketAddrV6.ip"))?.into(),
            value.port.try_into()?,
            0,
            0
        ))   
    }
}

impl From<std::net::Ipv6Addr> for Ipv6AddrMessage {
    fn from(value: std::net::Ipv6Addr) -> Self {
        let bits = value.to_bits();

        Self{
            high_bits: (bits >> 64) as u64,
            low_bits: bits as u64,
        }
    }
}
impl From<Ipv6AddrMessage> for std::net::Ipv6Addr{
    
    fn from(value: Ipv6AddrMessage) -> Self {
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
        let message = UuidMessage::from(uuid);
        uuid == uuid::Uuid::from(message)
    }

    #[test]
    fn uuid_conversion() {
        assert!(validate_uuid_conversion(uuid::Uuid::nil()));
        assert!(validate_uuid_conversion(uuid::Uuid::max()));
        assert!(validate_uuid_conversion(uuid::Uuid::now_v7()));
    }

    fn validate_socket_addr_conversion(socket_addr: net::SocketAddr) -> Result<bool, ProtoDeserializeError> {
        let message = SocketAddrMessage::from(socket_addr);
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