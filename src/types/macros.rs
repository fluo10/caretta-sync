macro_rules! database_impl {
    ($SelfT:ty) => {
        impl AsRef<sea_orm::DatabaseConnection> for $SelfT {
            fn as_ref(&self) -> &sea_orm::DatabaseConnection {
                &self.0
            }
        }
        impl From<sea_orm::DatabaseConnection> for $SelfT {
            fn from(value: sea_orm::DatabaseConnection) -> Self {
                Self(value)
            }
        }

        impl From<$SelfT> for sea_orm::DatabaseConnection {
            fn from(value: $SelfT) -> Self {
                value.0
            }
        }
    };
}

macro_rules! iroh_key_def {
    {
        SecretKey = ($SecretKey:ident,$IrohSecretKey:ty),
        PublicKey = ($PublicKey:ident,$IrohPublicKey:ty)
     } => {

        #[doc = concat!("A wrapper struct of [`ed25519_dalek::VerifyingKey`] compatible with [`", stringify!($IrohPublicKey), "`]")]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $PublicKey(ed25519_dalek::VerifyingKey);

        
        #[doc = concat!("A wrapper struct of [`ed25519_dalek::SigningKey`] compatible with [`", stringify!($IrohSecretKey), "`]")]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $SecretKey(ed25519_dalek::SigningKey);
    };
}

pub(crate) use iroh_key_def;

macro_rules! iroh_key_impl {
    {
        SecretKey = ( $SecretKey:ty, $IrohSecretKey:ty ),
        PublicKey = ( $PublicKey:ty, $IrohPublicKey:ty )
    } => {

        impl $SecretKey {
            #[cfg(feature = "server")]
            pub fn new() -> Self {
                ed25519_dalek::SigningKey::generate(&mut rand::rng()).into()
            }

            pub fn public_key(&self) -> $PublicKey {
                self.0.verifying_key().into()
            }
            pub fn as_bytes(&self) -> &[u8; 32] {
                self.0.as_bytes()
            }

            pub fn to_bytes(&self) -> [u8; 32] {
                self.0.to_bytes()
            }
            pub fn from_bytes(bytes: &[u8; 32]) -> Self {
                Self(ed25519_dalek::SigningKey::from_bytes(bytes))
            }
        }

        impl From<ed25519_dalek::SigningKey> for $SecretKey {
            fn from(value: ed25519_dalek::SigningKey) -> Self {
                Self(value)
            }
        }

        impl From<$SecretKey> for ed25519_dalek::SigningKey {
            fn from(value: $SecretKey) -> Self {
                value.0
            }
        }

        #[cfg(feature = "server")]
        impl From<$IrohSecretKey> for $SecretKey {
            fn from(value: $IrohSecretKey) -> Self {
                Self::from_bytes(&value.to_bytes())
            }
        }

        #[cfg(feature = "server")]
        impl From<$SecretKey> for $IrohSecretKey {
            fn from(value: $SecretKey) -> Self {
                Self::from_bytes(value.as_bytes())
            }
        }

        #[cfg(feature = "server")]
        impl From<$SecretKey> for sea_orm::Value {
            fn from(value: $SecretKey) -> Self {
                sea_orm::Value::Bytes(Some(Vec::from(value.as_bytes())))
            }
        }

        impl From<&[u8; 32]> for $SecretKey {
            fn from(value: &[u8; 32]) -> Self {
                Self::from_bytes(value)
            }
        }

        impl TryFrom<&[u8]> for $SecretKey {
            type Error = crate::types::KeyParsingError;
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let slice: [u8; 32] = value[0..32].try_into()?;
                Ok(Self::from_bytes(&slice))
            }
        }

        impl std::fmt::Display for $SecretKey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", &crate::util::encode_base32(&self.to_bytes()))
            }
        }

        impl std::str::FromStr for $SecretKey {
            type Err = crate::types::KeyParsingError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v = crate::util::decode_base32(s)?;
                let slice: &[u8;32] = v.as_slice().try_into()?;
                Ok(<$SecretKey>::from_bytes(slice))
            }
        }

        impl schemars::JsonSchema for $SecretKey {
            fn inline_schema() -> bool {
                true
            }
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($SecretKey).into()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                format!("{}::{}", module_path!(), Self::schema_name()).into()
            }
            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string",
                    "description": "base32 encoded secret key",
                    "pattern": "^[a-zA-Z0-9]{52}$"
                })
            }
        }

        #[cfg(feature = "server")]
        impl sea_orm::TryGetable for $SecretKey {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, sea_orm::TryGetError> {
                let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
                let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| sea_orm::DbErr::TryIntoErr {
                    from: stringify!(Vec<u8>),
                    into: stringify!(SecretKey),
                    source: std::sync::Arc::new(x),
                })?;
                Ok(<$SecretKey>::from_bytes(&slice))
            }
        }
        #[cfg(feature="server")]
        impl sea_orm::sea_query::ValueType for $SecretKey {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let vec = <Vec<u8> as sea_orm::sea_query::ValueType>::try_from(v)?;
                let key =
                    <$SecretKey as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
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
        #[cfg(feature="server")]
        impl sea_orm::sea_query::Nullable for $SecretKey {
            fn null() -> sea_orm::sea_query::Value {
                <Vec<u8> as sea_orm::sea_query::Nullable>::null()
            }
        }

        impl serde::Serialize for $SecretKey {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                let bytes = self.to_bytes();
                if serializer.is_human_readable() {
                    serializer.serialize_str(&crate::util::encode_base32(&bytes))
                } else {
                    serializer.serialize_bytes(&bytes)
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $SecretKey {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                use serde::de::Error as _;
                let bytes = if deserializer.is_human_readable() {
                    crate::util::decode_base32(&String::deserialize(deserializer)?).map_err(D::Error::custom)?
                } else {
                    Vec::<u8>::deserialize(deserializer)?
                };
                Ok(Self::from_bytes(bytes.as_slice().try_into().map_err(D::Error::custom)?))
            }
        }

        impl $PublicKey {
            pub const LENGTH:usize = 32;

            pub fn as_bytes(&self) -> &[u8; 32] {
                self.0.as_bytes()
            }

            pub fn to_bytes(&self) -> [u8; 32] {
                self.0.to_bytes()
            }
            
            pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self,crate::types::KeyParsingError> {
                Ok(ed25519_dalek::VerifyingKey::from_bytes(bytes).map(|x| Self(x))?)
            }
        }

        impl From<ed25519_dalek::VerifyingKey> for $PublicKey {
            fn from(value: ed25519_dalek::VerifyingKey) -> Self {
                Self(value)
            }
        }

        impl From<$PublicKey> for ed25519_dalek::VerifyingKey {
            fn from(value: $PublicKey) -> Self {
                value.0
            }
        }

        #[cfg(feature="server")]
        impl From<$IrohPublicKey> for $PublicKey {
            fn from(value: $IrohPublicKey) -> Self {
                Self::from_bytes(value.as_bytes()).unwrap()
            }
        }
        #[cfg(feature="server")]
        impl From<$PublicKey> for $IrohPublicKey {
            fn from(value: $PublicKey) -> Self {
                Self::from_bytes(value.as_bytes()).unwrap()
            }
        }
        #[cfg(feature="server")]
        impl From<$PublicKey> for sea_orm::Value {
            fn from(value: $PublicKey) -> Self {
                sea_orm::Value::Bytes(Some(Vec::from(value.as_bytes())))
            }
        }

        impl TryFrom<&[u8]> for $PublicKey {
            type Error = crate::types::KeyParsingError;
            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let slice: [u8; 32] = value[0..32].try_into()?;
                Ok(Self::from_bytes(&slice)?)
            }
        }

        impl std::fmt::Display for $PublicKey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", &crate::util::encode_base32(self.as_bytes()))
            }
        }

        impl std::str::FromStr for $PublicKey {
            type Err = crate::types::KeyParsingError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v = crate::util::decode_base32(s)?;
                let slice: &[u8;32] = v.as_slice().try_into()?;
                <$PublicKey>::from_bytes(slice)
            }
        }

         impl schemars::JsonSchema for $PublicKey {
            fn inline_schema() -> bool {
                true
            }
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($PublicKey).into()
            }
            fn schema_id() -> std::borrow::Cow<'static, str> {
                format!("{}::{}", module_path!(), Self::schema_name()).into()
            }
            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string",
                    "description": "base32 encoded public key",
                    "pattern": "^[a-zA-Z0-9]{52}$"
                })
            }
        }

        #[cfg(feature = "server")]
        impl sea_orm::TryGetable for $PublicKey {
            fn try_get_by<I: sea_orm::ColIdx>(
                res: &sea_orm::QueryResult,
                index: I,
            ) -> Result<Self, sea_orm::TryGetError> {
                let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
                let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| sea_orm::DbErr::TryIntoErr {
                    from: stringify!(Vec<u8>),
                    into: stringify!($PublicKey),
                    source: std::sync::Arc::new(x),
                })?;
                Ok(<$PublicKey>::from_bytes(&slice).map_err(|x| sea_orm::DbErr::TryIntoErr { from: stringify!(Vec<u8>), into: stringify!($PublicKey), source: std::sync::Arc::new(x) })?)
            }
        }
        #[cfg(feature = "server")]
        impl sea_orm::sea_query::ValueType for $PublicKey {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                let vec = <Vec<u8> as sea_orm::sea_query::ValueType>::try_from(v)?;
                let key =
                    <$PublicKey as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
                Ok(key)
            }
            fn type_name() -> String {
                stringify!($PublicKey).to_owned()
            }
            fn array_type() -> sea_orm_migration::prelude::ArrayType {
                sea_orm::sea_query::ArrayType::Bytes
            }
            fn column_type() -> sea_orm::ColumnType {
                sea_orm::sea_query::ColumnType::Blob
            }
        }
        #[cfg(feature = "server")]
        impl sea_orm::sea_query::Nullable for $PublicKey {
            fn null() -> sea_orm::Value {
                <Vec<u8> as sea_orm::sea_query::Nullable>::null()
            }
        }

        impl serde::Serialize for $PublicKey {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer {
                let bytes = self.as_bytes();
                if serializer.is_human_readable() {
                    serializer.serialize_str(&crate::util::encode_base32(bytes))
                } else {
                    serializer.serialize_bytes(bytes)
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $PublicKey {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                use serde::de::Error as _;
                let bytes = if deserializer.is_human_readable() {
                    crate::util::decode_base32(&String::deserialize(deserializer)?).map_err(D::Error::custom)?
                } else {
                    Vec::<u8>::deserialize(deserializer)?
                };
                Self::from_bytes(bytes.as_slice().try_into().map_err(D::Error::custom)?).map_err(D::Error::custom)
            }
        }


        #[cfg(test)]
        mod tests {
            use super::*;
            use std::sync::LazyLock;

            static SECRET_KEY: LazyLock<$SecretKey> = LazyLock::new(|| {
                <$SecretKey>::new()
            });
            static PUBLIC_KEY: LazyLock<$PublicKey> = LazyLock::new(|| {
                (*SECRET_KEY).public_key()
            });
            #[test]
            fn secret_key_json_schema() {
                let schema = serde_json::Value::from(<$SecretKey as schemars::JsonSchema>::json_schema(&mut schemars::SchemaGenerator::new(schemars::generate::SchemaSettings::openapi3())));
                let instance = serde_json::to_value(&*SECRET_KEY).unwrap();

                jsonschema::validate(&schema, &instance).unwrap();
            }

            #[test]
            fn secret_key_json_convertion() {
                let s = serde_json::to_string(&*SECRET_KEY).unwrap();
                let t: $SecretKey = serde_json::from_str(&s).unwrap();
                assert_eq!(t, *SECRET_KEY)
            }

            #[test]
            fn secret_key_cbor_conversion() {
                let mut v: Vec<u8> = Vec::new();
                ciborium::into_writer(&*SECRET_KEY, &mut v).unwrap();
                let t: $SecretKey = ciborium::from_reader(v.as_slice()).unwrap();
                assert_eq!(t, *SECRET_KEY)
            }


            #[test]
            fn public_key_jron_schema() {
                let schema = serde_json::Value::from(<$PublicKey as schemars::JsonSchema>::json_schema(&mut schemars::SchemaGenerator::new(schemars::generate::SchemaSettings::openapi3())));
                let instance = serde_json::to_value(&*PUBLIC_KEY).unwrap();

                jsonschema::validate(&schema, &instance).unwrap();
            }

                #[test]
            fn public_key_json_convertion() {
                let s = serde_json::to_string(&*PUBLIC_KEY).unwrap();
                let t: $PublicKey = serde_json::from_str(&s).unwrap();
                assert_eq!(t, *PUBLIC_KEY)
            }

            #[test]
            fn pubic_key_cbor_conversion() {
                let mut v: Vec<u8> = Vec::new();
                ciborium::into_writer(&*PUBLIC_KEY, &mut v).unwrap();
                let t: $PublicKey = ciborium::from_reader(v.as_slice()).unwrap();
                assert_eq!(t, *PUBLIC_KEY)
            }
        }


    }
}

pub(crate) use iroh_key_impl;
