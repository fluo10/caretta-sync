#[doc = concat!("A wrapper struct of [`ed25519_dalek::VerifyingKey`] compatible with [`", stringify!($IrohPublicKey), "`]")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DevicePublicKey(ed25519_dalek::VerifyingKey);

impl DevicePublicKey {
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

impl From<ed25519_dalek::VerifyingKey> for DevicePublicKey {
    fn from(value: ed25519_dalek::VerifyingKey) -> Self {
        Self(value)
    }
}

impl From<DevicePublicKey> for ed25519_dalek::VerifyingKey {
    fn from(value: DevicePublicKey) -> Self {
        value.0
    }
}

#[cfg(feature="server")]
impl From<iroh_docs::NamespacePublicKey> for DevicePublicKey {
    fn from(value: iroh_docs::NamespacePublicKey) -> Self {
        Self::from_bytes(value.as_bytes()).unwrap()
    }
}
#[cfg(feature="server")]
impl From<DevicePublicKey> for iroh_docs::NamespacePublicKey {
    fn from(value: DevicePublicKey) -> Self {
        Self::from_bytes(value.as_bytes()).unwrap()
    }
}


#[cfg(feature="server")]
impl From<DevicePublicKey> for iroh::PublicKey {
    fn from(value: DevicePublicKey) -> Self {
        Self::from_bytes(value.as_bytes()).unwrap()
    }
}

#[cfg(feature="server")]
impl From<iroh::PublicKey> for DevicePublicKey {
    fn from(value: iroh::PublicKey) -> Self {
        Self::from_bytes(value.as_bytes()).unwrap()
    }
}

#[cfg(feature="server")]
impl From<DevicePublicKey> for sea_orm::Value {
    fn from(value: DevicePublicKey) -> Self {
        sea_orm::Value::Bytes(Some(Vec::from(value.as_bytes())))
    }
}


impl TryFrom<&[u8]> for DevicePublicKey {
    type Error = crate::types::KeyParsingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice)?)
    }
}

impl std::fmt::Display for DevicePublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &crate::util::encode_base32(self.as_bytes()))
    }
}

impl std::str::FromStr for DevicePublicKey {
    type Err = crate::types::KeyParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = crate::util::decode_base32(s)?;
        let slice: &[u8;32] = v.as_slice().try_into()?;
        <DevicePublicKey>::from_bytes(slice)
    }
}

    impl schemars::JsonSchema for DevicePublicKey {
    fn inline_schema() -> bool {
        true
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        stringify!(DevicePublicKey).into()
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
impl sea_orm::TryGetable for DevicePublicKey {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let vec = <Vec<u8> as sea_orm::TryGetable>::try_get_by(res, index)?;
        let slice: [u8; 32] = vec[0..32].try_into().map_err(|x| sea_orm::DbErr::TryIntoErr {
            from: stringify!(Vec<u8>),
            into: stringify!(DevicePublicKey),
            source: std::sync::Arc::new(x),
        })?;
        Ok(<DevicePublicKey>::from_bytes(&slice).map_err(|x| sea_orm::DbErr::TryIntoErr { from: stringify!(Vec<u8>), into: stringify!(DevicePublicKey), source: std::sync::Arc::new(x) })?)
    }
}
#[cfg(feature = "server")]
impl sea_orm::sea_query::ValueType for DevicePublicKey {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        let vec = <Vec<u8> as sea_orm::sea_query::ValueType>::try_from(v)?;
        let key =
            <DevicePublicKey as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
        Ok(key)
    }
    fn type_name() -> String {
        stringify!(DevicePublicKey).to_owned()
    }
    fn array_type() -> sea_orm_migration::prelude::ArrayType {
        sea_orm::sea_query::ArrayType::Bytes
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::Blob
    }
}
#[cfg(feature = "server")]
impl sea_orm::sea_query::Nullable for DevicePublicKey {
    fn null() -> sea_orm::Value {
        <Vec<u8> as sea_orm::sea_query::Nullable>::null()
    }
}

impl serde::Serialize for DevicePublicKey {
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

impl<'de> serde::Deserialize<'de> for DevicePublicKey {
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

#[doc = concat!("A wrapper struct of [`ed25519_dalek::SigningKey`] compatible with [`", stringify!($IrohSecretKey), "`]")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceSecretKey(ed25519_dalek::SigningKey);

impl DeviceSecretKey {
    #[cfg(feature = "server")]
    pub fn new() -> Self {
        ed25519_dalek::SigningKey::generate(&mut rand::rng()).into()
    }

    pub fn public_key(&self) -> DevicePublicKey {
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

impl From<ed25519_dalek::SigningKey> for DeviceSecretKey {
    fn from(value: ed25519_dalek::SigningKey) -> Self {
        Self(value)
    }
}

impl From<DeviceSecretKey> for ed25519_dalek::SigningKey {
    fn from(value: DeviceSecretKey) -> Self {
        value.0
    }
}

#[cfg(feature = "server")]
impl From<iroh::SecretKey> for DeviceSecretKey {
    fn from(value: iroh::SecretKey) -> Self {
        Self::from_bytes(&value.to_bytes())
    }
}

#[cfg(feature = "server")]
impl From<DeviceSecretKey> for iroh::SecretKey {
    fn from(value: DeviceSecretKey) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

#[cfg(feature = "server")]
impl From<iroh_docs::NamespaceSecret> for DeviceSecretKey {
    fn from(value: iroh_docs::NamespaceSecret) -> Self {
        Self::from_bytes(&value.to_bytes())
    }
}

#[cfg(feature = "server")]
impl From<DeviceSecretKey> for iroh_docs::NamespaceSecret {
    fn from(value: DeviceSecretKey) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

#[cfg(feature = "server")]
impl From<DeviceSecretKey> for sea_orm::Value {
    fn from(value: DeviceSecretKey) -> Self {
        sea_orm::Value::Bytes(Some(Vec::from(value.as_bytes())))
    }
}

impl From<&[u8; 32]> for DeviceSecretKey {
    fn from(value: &[u8; 32]) -> Self {
        Self::from_bytes(value)
    }
}

impl TryFrom<&[u8]> for DeviceSecretKey {
    type Error = crate::types::KeyParsingError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice: [u8; 32] = value[0..32].try_into()?;
        Ok(Self::from_bytes(&slice))
    }
}

impl std::fmt::Display for DeviceSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &crate::util::encode_base32(&self.to_bytes()))
    }
}

impl std::str::FromStr for DeviceSecretKey {
    type Err = crate::types::KeyParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = crate::util::decode_base32(s)?;
        let slice: &[u8;32] = v.as_slice().try_into()?;
        Ok(<DeviceSecretKey>::from_bytes(slice))
    }
}

impl schemars::JsonSchema for DeviceSecretKey {
    fn inline_schema() -> bool {
        true
    }
    fn schema_name() -> std::borrow::Cow<'static, str> {
        stringify!(DeviceSecretKey).into()
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
impl sea_orm::TryGetable for DeviceSecretKey {
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
        Ok(<DeviceSecretKey>::from_bytes(&slice))
    }
}
#[cfg(feature="server")]
impl sea_orm::sea_query::ValueType for DeviceSecretKey {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        let vec = <Vec<u8> as sea_orm::sea_query::ValueType>::try_from(v)?;
        let key =
            <DeviceSecretKey as TryFrom<&[u8]>>::try_from(&vec[0..32]).map_err(|_| sea_orm::sea_query::ValueTypeErr)?;
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
impl sea_orm::sea_query::Nullable for DeviceSecretKey {
    fn null() -> sea_orm::sea_query::Value {
        <Vec<u8> as sea_orm::sea_query::Nullable>::null()
    }
}

impl serde::Serialize for DeviceSecretKey {
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

impl<'de> serde::Deserialize<'de> for DeviceSecretKey {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    static SECRET_KEY: LazyLock<DeviceSecretKey> = LazyLock::new(|| {
        <DeviceSecretKey>::new()
    });
    static PUBLIC_KEY: LazyLock<DevicePublicKey> = LazyLock::new(|| {
        (*SECRET_KEY).public_key()
    });
    #[test]
    fn secret_key_json_schema() {
        let schema = serde_json::Value::from(<DeviceSecretKey as schemars::JsonSchema>::json_schema(&mut schemars::SchemaGenerator::new(schemars::generate::SchemaSettings::openapi3())));
        let instance = serde_json::to_value(&*SECRET_KEY).unwrap();

        jsonschema::validate(&schema, &instance).unwrap();
    }

    #[test]
    fn secret_key_json_convertion() {
        let s = serde_json::to_string(&*SECRET_KEY).unwrap();
        let t: DeviceSecretKey = serde_json::from_str(&s).unwrap();
        assert_eq!(t, *SECRET_KEY)
    }

    #[test]
    fn secret_key_cbor_conversion() {
        let mut v: Vec<u8> = Vec::new();
        ciborium::into_writer(&*SECRET_KEY, &mut v).unwrap();
        let t: DeviceSecretKey = ciborium::from_reader(v.as_slice()).unwrap();
        assert_eq!(t, *SECRET_KEY)
    }


    #[test]
    fn public_key_jron_schema() {
        let schema = serde_json::Value::from(<DevicePublicKey as schemars::JsonSchema>::json_schema(&mut schemars::SchemaGenerator::new(schemars::generate::SchemaSettings::openapi3())));
        let instance = serde_json::to_value(&*PUBLIC_KEY).unwrap();

        jsonschema::validate(&schema, &instance).unwrap();
    }

        #[test]
    fn public_key_json_convertion() {
        let s = serde_json::to_string(&*PUBLIC_KEY).unwrap();
        let t: DevicePublicKey = serde_json::from_str(&s).unwrap();
        assert_eq!(t, *PUBLIC_KEY)
    }

    #[test]
    fn pubic_key_cbor_conversion() {
        let mut v: Vec<u8> = Vec::new();
        ciborium::into_writer(&*PUBLIC_KEY, &mut v).unwrap();
        let t: DevicePublicKey = ciborium::from_reader(v.as_slice()).unwrap();
        assert_eq!(t, *PUBLIC_KEY)
    }
}