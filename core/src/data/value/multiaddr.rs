use std::str::FromStr;

use libp2p::Multiaddr;
use sea_orm::{sea_query::ValueTypeErr, DbErr};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MultiaddrValue(Multiaddr);

impl From<Multiaddr> for MultiaddrValue {
    fn from(source: Multiaddr) -> Self {
        Self(source)
    }
}
impl From<MultiaddrValue> for Multiaddr {
    fn from(source: MultiaddrValue) -> Self {
        source.0
    }
}

impl From<MultiaddrValue> for sea_orm::Value {
    fn from(value: MultiaddrValue) -> Self {
        Self::from(value.0.to_string())
    }
}

impl sea_orm::TryGetable for MultiaddrValue {
    fn try_get_by<I: sea_orm::ColIdx>(res: &sea_orm::QueryResult, idx: I)
        -> std::result::Result<Self, sea_orm::TryGetError> {
        match <String as sea_orm::TryGetable>::try_get_by(res, idx){
            Ok(x) => match Multiaddr::from_str(&x) {
                Ok(y) => Ok(Self(y)),
                Err(_) => Err(DbErr::Type("Multiaddr".to_string()).into()),
            },
            Err(x) => Err(x),
        }
    }
}

impl sea_orm::sea_query::ValueType for MultiaddrValue {
    fn try_from(v: sea_orm::Value) -> std::result::Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match <String as sea_orm::sea_query::ValueType>::try_from(v) {
            Ok(x) => match Multiaddr::from_str(&x) {
                Ok(y) => Ok(Self(y)),
                Err(_) => Err(ValueTypeErr{}),
            },
            Err(e) => Err(e)
        }
    }

    fn type_name() -> std::string::String {
        stringify!(MultiaddrValue).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Text
    }
}

impl sea_orm::sea_query::Nullable for MultiaddrValue {
    fn null() -> sea_orm::Value {
        <String as sea_orm::sea_query::Nullable>::null()
    }
}