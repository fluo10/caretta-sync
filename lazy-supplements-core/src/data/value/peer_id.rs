use std::str::FromStr;

use libp2p::PeerId;
use sea_orm::{sea_query::ValueTypeErr, DbErr};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct PeerIdValue(PeerId);

impl<'de> Deserialize<'de> for PeerIdValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        Self::from_str(&String::deserialize(deserializer)?).or(Err(<D::Error as serde::de::Error>::custom("fail to parse PeerId")))
    
    }
}

impl Serialize for PeerIdValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl FromStr for PeerIdValue{
    type Err = libp2p::identity::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PeerId::from_str(s)?))
    }
}

impl ToString for PeerIdValue {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<PeerId> for PeerIdValue {
    fn from(source: PeerId) -> Self {
        Self(source)
    }
}
impl From<PeerIdValue> for PeerId {
    fn from(source: PeerIdValue) -> Self {
        source.0
    }
}

impl From<PeerIdValue> for sea_orm::Value {
    fn from(value: PeerIdValue) -> Self {
        Self::from(value.0.to_string())
    }
}

impl sea_orm::TryGetable for PeerIdValue {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, idx: I)
        -> std::result::Result<Self, sea_orm::TryGetError> {
        match <String as sea_orm::TryGetable>::try_get_by(res, idx){
            Ok(x) => match PeerId::from_str(&x) {
                Ok(y) => Ok(Self(y)),
                Err(_) => Err(DbErr::Type("PeerId".to_string()).into()),
            },
            Err(x) => Err(x),
        }
    }
}

impl sea_orm::sea_query::ValueType for PeerIdValue {
    fn try_from(v: sea_orm::Value) -> std::result::Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match <String as sea_orm::sea_query::ValueType>::try_from(v) {
            Ok(x) => match PeerId::from_str(&x) {
                Ok(y) => Ok(Self(y)),
                Err(_) => Err(ValueTypeErr{}),
            },
            Err(e) => Err(e)
        }
    }

    fn type_name() -> std::string::String {
        stringify!(PeerIdValue).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Text
    }
}

impl sea_orm::sea_query::Nullable for PeerIdValue {
    fn null() -> sea_orm::Value {
        <String as sea_orm::sea_query::Nullable>::null()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{test_cbor_serialize_deserialize, test_toml_serialize_deserialize};

    use super::*;

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    struct PeerIdValueWrapper {
        content: PeerIdValue

    }
    #[test]
    fn test_serialize_deserialize() {
        let peer_id= PeerIdValueWrapper{content: PeerIdValue::from(PeerId::random())};
        let x = toml::to_string(&peer_id).unwrap();
        assert_eq!(peer_id.content, toml::from_str::<PeerIdValueWrapper>(&x).unwrap().content)
    }
}
