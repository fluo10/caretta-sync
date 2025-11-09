use std::{fmt::Display, str::FromStr};

/// Binary data parsed from/ encode to Base32 string
#[derive(Clone, Debug, PartialEq)]
pub struct Base32Bytes(Vec<u8>);

impl AsRef<[u8]> for Base32Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&[u8]> for Base32Bytes {
    fn from(value: &[u8]) -> Self {
        Self(Vec::from(value))
    }
}
impl From<Vec<u8>> for Base32Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
impl From<Base32Bytes> for Vec<u8> {
    fn from(value: Base32Bytes) -> Self {
        value.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Base32BytesError {
    #[error("Base32 Decode error")]
    Decode,
}

impl FromStr for Base32Bytes {
    type Err = Base32BytesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base32::decode(base32::Alphabet::Crockford, s)
            .ok_or(Base32BytesError::Decode)
            .map(|x| Base32Bytes(x))
    }
}

impl Display for Base32Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        base32::encode(base32::Alphabet::Crockford, &self.0[..])
            .to_ascii_lowercase()
            .fmt(f)
    }
}

#[cfg(feature = "desktop")]
mod desktop {
    use ::serde::{Deserialize, Serialize, de::Error};

    use super::*;

    impl Serialize for Base32Bytes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&self.to_string())
            } else {
                serializer.serialize_bytes(self.as_ref())
            }
        }
    }

    impl<'de> Deserialize<'de> for Base32Bytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                String::deserialize(deserializer).map(|x| {
                    Self::from_str(&x).map_err(|e| {
                        D::Error::invalid_value(::serde::de::Unexpected::Str(&x), &"Base32 string")
                    })
                })?
            } else {
                <&[u8]>::deserialize(deserializer).map(|x| Self::from(x))
            }
        }
    }
}
