tonic::include_proto!("caretta_sync.types.url");

use crate::proto::error::ProtoDeserializeError;

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
