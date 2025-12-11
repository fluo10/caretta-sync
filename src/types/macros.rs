macro_rules! def_new_type {
    { 
        Self = $SelfT:ident,
        Inner = $Inner:ty
    } => {
        #[doc = concat!("A wrapper struct of [`", stringify!($Inner), "`]")]
        ///
        /// 
        ///
        /// # Examples
        /// 
        /// ## Sea ORM
        /// ```
        /// # use sea_orm::entity::prelude::*;
        #[doc = concat!("use caretta_brain::types::", stringify!($Inner), ";")]
        /// #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        /// #[sea_orm(table_name = "example")]
        /// pub struct Model {
        ///     #[sea_orm(primary_key)]
        ///     pub id: u32,
        #[doc = concat!("pub value: ", stringify!($Self), ",")] 
        /// }
        /// # #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        /// # pub enum Relation {}
        /// #
        /// # impl ActiveModelBehavior for ActiveModel{}
        /// ```
        #[derive(Clone, Debug)]
        pub struct $SelfT($Inner);
    };
}

macro_rules! def_iroh_public_key {
    { 
        Self = $SelfT:ident,
        Inner = $Inner:ty,
        TryIntoError = $TryIntoError:ident,
        InvalidBytesValueInner = $InvalidBytesValueInner:ty
    } => {
        crate::types::macros::def_new_type!(
            Self = $SelfT,
            Inner = $Inner
        );

        #[derive(Debug, thiserror::Error)]
        pub enum $TryIntoError{
            #[error("Expected base32 string, found {0}")]
            InvalidBase32String(#[from] DecodeBase32Error),
            #[error("invalid length {0}")]
            InvalidBytesLength(#[from] TryFromSliceError),
            #[error("invalid value {0}")]
            InvalidBytesValue(#[from] $InvalidBytesValueInner),
        }
    };
}


macro_rules! def_iroh_secret_key {
    { 
        Self = $SelfT:ident,
        Inner = $Inner:ty,
        TryIntoError = $TryIntoError:ident
    } => {
        crate::types::macros::def_new_type!(
            Self = $SelfT,
            Inner = $Inner
        );
        #[derive(Debug, thiserror::Error)]
        pub enum $TryIntoError{
            #[error("Expected base32 string, found {0}")]
            InvalidString(#[from] crate::util::DecodeBase32Error),
            #[error("Invalid slice length: {0}")]
            InvalidSliceLength(#[from] std::array::TryFromSliceError),
        }
    };
}

macro_rules! impl_iroh_public_key {
    {
        Self = $SelfT:ty,
        Inner = $Inner:ty,
        TryIntoError = $TryIntoError:ty,
    } => {
        impl $SelfT {
            pub const LENGTH:usize = 32;
            pub fn into_inner(self) -> $Inner {
                self.0
            }
            pub fn as_bytes(&self) -> &[u8; 32] {
                self.0.as_bytes()
            }
            pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self,$TryIntoError> {
                Ok(<$Inner>::from_bytes(bytes).map(|x| Self(x))?)
            }
        }

        impl From<$Inner> for $SelfT {
            fn from(value: $Inner) -> Self {
                Self(value)
            }
        }

        impl From<$SelfT> for $Inner {
            fn from(value: $SelfT) -> Self {
                value.into_inner()
            }
        }

        impl From<$SelfT> for sea_orm::Value {
            fn from(value: $SelfT) -> Self {
                Value::Bytes(Some(Vec::from(value.as_bytes())))
            }
        }

        impl PartialEq for $SelfT {
            fn eq(&self, other: &$SelfT) -> bool {
                self.0 ==  other.0
            }
        }
        impl Eq for $SelfT {}

        impl TryFrom<&[u8]> for $SelfT {
            type Error = $TryIntoError;
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let slice: [u8; 32] = value[0..32].try_into()?;
                Ok(Self::from_bytes(&slice)?)
            }
        }

        impl std::fmt::Display for $SelfT {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", &crate::util::encode_base32(self.as_bytes()))
            }
        }

        impl std::str::FromStr for $SelfT {
            type Err = $TryIntoError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v = crate::util::decode_base32(s)?;
                let slice: &[u8;32] = v.as_slice().try_into()?;
                <$SelfT>::from_bytes(slice)
            }
        }

        impl TryGetable for $SelfT {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, sea_orm::TryGetError> {
                let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
                let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| DbErr::TryIntoErr {
                    from: stringify!(Vec<u8>),
                    into: stringify!($SelfT),
                    source: Arc::new(x),
                })?;
                Ok(<$SelfT>::from_bytes(&slice).map_err(|x| DbErr::TryIntoErr { from: stringify!(Vec<u8>), into: stringify!($SelfT), source: Arc::new(x) })?)
            }
        }

        impl ValueType for $SelfT {
            fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
                let vec = <Vec<u8> as ValueType>::try_from(v)?;
                let key =
                    <$SelfT as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| ValueTypeErr)?;
                Ok(key)
            }
            fn type_name() -> String {
                stringify!($SelfT).to_owned()
            }
            fn array_type() -> sea_orm_migration::prelude::ArrayType {
                sea_orm::sea_query::ArrayType::Bytes
            }
            fn column_type() -> sea_orm::ColumnType {
                sea_orm::sea_query::ColumnType::Blob
            }
        }

        impl sea_orm::sea_query::Nullable for $SelfT {
            fn null() -> Value {
                <Vec<u8> as Nullable>::null()
            }
        }

        impl Serialize for $SelfT {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                todo!()
            }
        }

        impl<'de> Deserialize<'de> for $SelfT {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                todo!()
            }
        }
    }
}

macro_rules! impl_iroh_secret_key {
    {
        Self = $SelfT:ty,
        Inner = $Inner:ty,
        TryIntoError = $TryIntoError:ty,
        new = $new:path
    } => {
        impl $SelfT {
            pub fn new() -> Self {
                Self($new(&mut rand::rng()))
            }

            pub fn to_bytes(&self) -> [u8; 32] {
                self.0.to_bytes()
            }
            pub fn from_bytes(bytes: &[u8; 32]) -> Self {
                Self(<$Inner>::from_bytes(bytes))
            }
        }

        impl PartialEq for $SelfT {
            fn eq(&self, other: &Self) -> bool {
                self.to_bytes().eq(&other.to_bytes())
            }
        }
        impl Eq for $SelfT {}

        impl From<$Inner> for $SelfT {
            fn from(value: $Inner) -> Self {
                Self(value)
            }
        }

        impl From<$SelfT> for $Inner {
            fn from(value: $SelfT) -> Self {
                value.0
            }
        }

        impl From<$SelfT> for sea_orm::Value {
            fn from(value: $SelfT) -> Self {
                Value::Bytes(Some(Vec::from(&value.to_bytes())))
            }
        }

        impl From<&[u8; 32]> for $SelfT {
            fn from(value: &[u8; 32]) -> Self {
                Self::from_bytes(value)
            }
        }

        impl TryFrom<&[u8]> for $SelfT {
            type Error = $TryIntoError;
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let slice: [u8; 32] = value[0..32].try_into()?;
                Ok(Self::from_bytes(&slice))
            }
        }
        
        impl std::fmt::Display for $SelfT {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", &crate::util::encode_base32(&self.to_bytes()))
            }
        }

        impl std::str::FromStr for $SelfT {
            type Err = $TryIntoError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v = crate::util::decode_base32(s)?;
                let slice: &[u8;32] = v.as_slice().try_into()?;
                Ok(<$SelfT>::from_bytes(slice))
            }
        }

        impl TryGetable for $SelfT {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, sea_orm::TryGetError> {
                let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
                let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| DbErr::TryIntoErr {
                    from: stringify!(Vec<u8>),
                    into: stringify!(SecretKey),
                    source: Arc::new(x),
                })?;
                Ok(<$SelfT>::from_bytes(&slice))
            }
        }

        impl ValueType for $SelfT {
            fn try_from(v: Value) -> Result<Self, sea_orm_migration::prelude::ValueTypeErr> {
                let vec = <Vec<u8> as ValueType>::try_from(v)?;
                let key =
                    <$SelfT as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| ValueTypeErr)?;
                Ok(key)
            }
            fn type_name() -> String {
                stringify!(SecretKey).to_owned()
            }
            fn array_type() -> sea_orm_migration::prelude::ArrayType {
                sea_orm::sea_query::ArrayType::Bytes
            }
            fn column_type() -> sea_orm::ColumnType {
                sea_orm::sea_query::ColumnType::Blob
            }
        }

        impl sea_orm::sea_query::Nullable for $SelfT {
            fn null() -> Value {
                <Vec<u8> as Nullable>::null()
            }
        }

        impl Serialize for $SelfT {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                todo!()
            }
        }

        impl<'de> Deserialize<'de> for $SelfT {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                todo!()
            }
        }
    }
}



pub(crate) use def_iroh_public_key;
pub(crate) use def_iroh_secret_key;
pub(crate) use impl_iroh_public_key;
pub(crate) use impl_iroh_secret_key;
pub(crate) use def_new_type;