//! Ser/de [`DateTime<Local>`]
//! 
//! Intended for use with serde's with attribute.
//! 
//! # Example
//! ```
//! # use caretta_sync_core::serde::byte_array_option;
//! # use chrono::{DateTime, NaiveDate, Utc};
//! # use serde::{Deserialize, Serialize};
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "byte_array_option")]
//!     bytes: Option<[u8;4]>
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let bytes = Some(*b"abcd");
//! let foo = Foo {bytes};
//! let as_string = toml::to_string(&foo)?;
//! assert_eq!(as_string, "bytes = \"c5h66s0\"\n");
//! 
//! let from_string = toml::from_str(&as_string)?;
//! assert_eq!(foo, from_string);
//! 
//! let mut as_bytes = Vec::new();
//! ciborium::into_writer(&foo, &mut as_bytes)?;
//! assert_eq!(as_bytes, vec![161, 101, 98, 121, 116, 101, 115, 132, 24, 97, 24, 98, 24, 99, 24, 100]);
//! 
//! let from_bytes: Foo = ciborium::from_reader(as_bytes.as_slice())?;
//! assert_eq!(foo, from_bytes);
//! # Ok(())
//! # }
//! 
//! ```

use std::fmt;

use serde::{de::{self, Error as _, Expected, Unexpected, Visitor}, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::util::{decode_base32, encode_base32};

pub fn serialize<S, const N:usize> (
    value: &Option<[u8;N]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    match value.as_ref() {
        Some(bytes) => {
            if serializer.is_human_readable() {
                serializer.serialize_some(&encode_base32(bytes))
            } else {
                serializer.serialize_some(&bytes[..])
            }
        },
        None => serializer.serialize_none()
    }
}

pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<Option<[u8;N]>, D::Error>
where 
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionByteArrayVisitor::<N>)
}

struct OptionByteArrayVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for OptionByteArrayVisitor<N> {
    type Value = Option<[u8;N]>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result{
        formatter.write_str("a byte array or none")
    }

    fn visit_some<D>(self, deserializer:D) -> Result<Self::Value, D::Error>
    where 
        D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let s = String::deserialize(deserializer)?;
                let v = decode_base32(&s).map_err(D::Error::custom)?;
                Ok(Some(v.as_slice().try_into().map_err(D::Error::custom)?))
            } else {
                let v = Vec::<u8>::deserialize(deserializer)?;
                Ok(Some(v.as_slice().try_into().map_err(D::Error::custom)?))
            }
        }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where 
        E: de::Error,
    {
        Ok(None)
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where 
        E: de::Error,
    {
        Ok(None)
    }
}
