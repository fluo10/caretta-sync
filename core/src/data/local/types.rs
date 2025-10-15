use iroh::PublicKey;
use sea_orm::{
    DbErr, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};

/// A wrapper of iroh::PublicKey to read/write with sea-orm
/// Saved as blob.
///
/// # Examples
/// ```
/// # use sea_orm::entity::prelude::*;
/// use caretta_sync_core::data::local::types::PublicKeyBlob;
/// #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
/// #[sea_orm(table_name = "public_key_example")]
/// pub struct Model {
///     #[sea_orm(primary_key)]
///     pub id: u32,
///     pub public_key: PublicKeyBlob
/// }
/// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
/// # pub enum Relation {}
/// #
/// # impl ActiveModelBehavior for ActiveModel{}
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicKeyBlob(PublicKey);

impl From<PublicKey> for PublicKeyBlob {
    fn from(value: PublicKey) -> Self {
        Self(value)
    }
}

impl From<PublicKeyBlob> for PublicKey {
    fn from(value: PublicKeyBlob) -> Self {
        value.0
    }
}

impl From<PublicKeyBlob> for Vec<u8> {
    fn from(value: PublicKeyBlob) -> Self {
        Vec::from(value.0.as_bytes())
    }
}

impl TryFrom<Vec<u8>> for PublicKeyBlob {
    type Error = DbErr;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let arr: [u8; 32] = value[0..32].try_into().map_err(|e| DbErr::TryIntoErr {
            from: stringify!(Vec<u8>),
            into: stringify!(PublicKeyBlob),
            source: Box::new(e),
        })?;
        match PublicKey::from_bytes(&arr) {
            Ok(x) => Ok(Self(x)),
            Err(e) => Err(DbErr::TryIntoErr {
                from: stringify!(Vec<u8>),
                into: stringify!(PublicKeyBlob),
                source: Box::new(e),
            }),
        }
    }
}

impl From<PublicKeyBlob> for sea_orm::Value {
    fn from(value: PublicKeyBlob) -> Self {
        Value::Bytes(Some(Box::new(value.into())))
    }
}

impl TryGetable for PublicKeyBlob {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
        <Self as TryFrom<Vec<u8>>>::try_from(vec).map_err(|e| e.into())
    }
}

impl ValueType for PublicKeyBlob {
    fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
        let vec = <Vec<u8> as ValueType>::try_from(v)?;
        let key = <PublicKeyBlob as TryFrom<Vec<u8>>>::try_from(vec).map_err(|_| ValueTypeErr)?;
        Ok(key)
    }
    fn type_name() -> String {
        stringify!(PublicKeyBlob).to_owned()
    }
    fn array_type() -> sea_orm_migration::prelude::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}

impl sea_orm::sea_query::Nullable for PublicKeyBlob {
    fn null() -> Value {
        <Vec<u8> as Nullable>::null()
    }
}
