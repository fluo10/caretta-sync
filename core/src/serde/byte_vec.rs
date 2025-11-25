//! Ser/de [`DateTime<Local>`]
//! 
//! Intended for use with serde's with attribute.
//! 
//! # Example
//! ```
//! # use caretta_sync_core::serde::byte_vec;
//! # use chrono::{DateTime, NaiveDate, Utc};
//! # use serde::{Deserialize, Serialize};
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "byte_vec")]
//!     bytes: Vec<u8> 
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let bytes = Vec::from(b"abcd");
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

use std::time::Duration;

use serde::{de::{Expected, Unexpected, Error as _}, {ser::{Error as _}}, Deserialize, Deserializer, Serialize, Serializer};

use crate::util::{decode_base32, encode_base32};

pub fn serialize<S> (
    value: &Vec<u8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    if serializer.is_human_readable() {
        encode_base32(value.as_slice()).serialize(serializer)
    } else {
        value.serialize(serializer)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where 
    D: Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let s = String::deserialize(deserializer)?;
        decode_base32(&s).map_err(D::Error::custom)
    } else {
        Vec::<u8>::deserialize(deserializer)
    }
}