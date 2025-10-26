use crate::proto::ProtoDeserializeError;

tonic::include_proto!("caretta_sync.types.uuid");

impl From<uuid::Uuid> for Uuid {
    fn from(value: uuid::Uuid) -> Self {
        let pair = value.as_u64_pair();
        Self{
            high_bits: pair.0,
            low_bits: pair.1,
        }
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(value: Uuid) -> Self {
        Self::from_u64_pair(value.high_bits, value.low_bits)
    }
}