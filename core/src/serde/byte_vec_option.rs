//! Ser/de [`DateTime<Local>`]
//! 
//! Intended for use with serde's with attribute.
//! 
//! # Example
//! ```
//! # use caretta_sync_core::serde::byte_vec_option;
//! # use chrono::{DateTime, NaiveDate, Utc};
//! # use serde::{Deserialize, Serialize};
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "byte_vec_option")]
//!     bytes: Option<Vec<u8>> 
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let bytes = Some(Vec::from(b"abcd"));
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

use std::{fmt, time::Duration};

use serde::{de::{self, Error as _, Expected, Unexpected}, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::util::{decode_base32, encode_base32};

pub fn serialize<S> (
    value: &Option<Vec<u8>>,
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
                serializer.serialize_some(bytes.as_slice())
            }
        },
        None => serializer.serialize_none()
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where 
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionByteVecVisitor)
}

struct OptionByteVecVisitor;

impl<'de> de::Visitor<'de> for OptionByteVecVisitor {
    type Value = Option<Vec<u8>>;
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
                Ok(Some(v))
            } else {
                let v = Vec::<u8>::deserialize(deserializer)?;
                Ok(Some(v))
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