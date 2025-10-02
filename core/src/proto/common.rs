use super::*;
tonic::include_proto!("caretta_sync.common");

use crate::proto::{error::{ProtoDeserializeError, ProtoSerializeError}};

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        let (first_half, second_half) = value.as_u64_pair();
        Self {
            high_bits: first_half,
            low_bits: second_half
        }
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        uuid::Uuid::from_u64_pair(value.high_bits, value.low_bits)
    }
}



impl From<url::Url> for Url {
    fn from(value: url::Url) -> Self {
        todo!()
    }
}

impl TryFrom<Url> for url::Url {
    type Error = ProtoDeserializeError;
    fn try_from(value: Url) -> Result<Self, Self::Error> {
        todo!()
    }
} 

#[cfg(test)]
mod tests {
    use std::{net::{self, Ipv4Addr}, u16};

    use super::*;
    fn validate_uuid_conversion(uuid: uuid::Uuid) -> bool{
        let message = Uuid::from(uuid);
        uuid == uuid::Uuid::from(message)
    }

    #[test]
    fn uuid_conversion() {
        assert!(validate_uuid_conversion(uuid::Uuid::nil()));
        assert!(validate_uuid_conversion(uuid::Uuid::max()));
        assert!(validate_uuid_conversion(uuid::Uuid::now_v7()));
    }

}