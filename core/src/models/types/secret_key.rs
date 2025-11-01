use std::array::TryFromSliceError;

use iroh::SecretKey;
use sea_orm::{
    sea_query::{Nullable, ValueType, ValueTypeErr}, DbErr, TryGetError, TryGetable, Value
};

/// A wrapper of iroh::SecretKey to read/write with sea-orm
/// 
/// Saved as blob.
///
/// # Examples
/// ```
/// # use sea_orm::entity::prelude::*;
/// use caretta_sync_core::models::types::SecretKeyBlob;
/// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
/// #[sea_orm(table_name = "secret_key_example")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: u32,
///     pub secret_key: SecretKeyBlob
/// }
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// # pub enum Relation {}
/// #
/// # impl ActiveModelBehavior for ActiveModel{}
/// ```
#[derive(Clone, Debug,)]
pub struct SecretKeyBlob(SecretKey);

impl SecretKeyBlob {
    pub fn to_bytes(&self) -> [u8;32] {
        self.0.to_bytes()
    } 
    pub fn from_bytes(bytes: &[u8;32]) -> Self {
        Self(SecretKey::from_bytes(bytes))
    }
}

impl PartialEq for SecretKeyBlob {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().eq(&other.to_bytes())
    }
}


impl From<SecretKey> for SecretKeyBlob {
    fn from(value: SecretKey) -> Self {
        Self(value)
    }
}

impl From<SecretKeyBlob> for SecretKey {
    fn from(value: SecretKeyBlob) -> Self {
        value.0
    }
}

impl From<SecretKeyBlob> for sea_orm::Value {
    fn from(value: SecretKeyBlob) -> Self {
        Value::Bytes(Some(Box::new(Vec::from(&value.to_bytes()))))
    }
}

impl From<&[u8;32]> for SecretKeyBlob {
    fn from(value: &[u8;32]) -> Self {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for SecretKeyBlob {
    type Error = TryFromSliceError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice: [u8;32] = value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice)) 
    }
}

impl TryGetable for SecretKeyBlob {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
        let slice: [u8;32] =  vec[0..32].try_into().map_err(|x| DbErr::TryIntoErr {
            from: stringify!(Vec<u8>),
            into: stringify!(SecretKeyBlob),
            source: Box::new(x)
        })?;
        Ok(SecretKeyBlob::from_bytes(&slice))
    }
}

impl ValueType for SecretKeyBlob {
    fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
        let vec = <Vec<u8> as ValueType>::try_from(v)?;
        let key = <SecretKeyBlob as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| ValueTypeErr)?;
        Ok(key)
    }
    fn type_name() -> String {
        stringify!(SecretKeyBlob).to_owned()
    }
    fn array_type() -> sea_orm_migration::prelude::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}

impl sea_orm::sea_query::Nullable for SecretKeyBlob {
    fn null() -> Value {
        <Vec<u8> as Nullable>::null()
    }
}
