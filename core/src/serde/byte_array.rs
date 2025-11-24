
//! Ser/de [`DateTime<Local>`]
//! 
//! Intended for use with serde's with attribute.
//! 
//! # Example
//! ```
//! # use caretta_sync_core::serde::byte_array;
//! # use chrono::{DateTime, NaiveDate, Utc};
//! # use serde::{Deserialize, Serialize};
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "byte_array")]
//!     bytes: [u8;4], 
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let bytes: [u8;4] = *b"abcd";
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

pub fn serialize<S, const N:usize> (
    value: &[u8;N],
    serializer: S,
) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
{
    if serializer.is_human_readable() {
        encode_base32(value).serialize(serializer)
    } else {
        value.serialize(serializer)
    }
}

pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8;N], D::Error>
where 
    D: Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let s = String::deserialize(deserializer)?;
        let v = decode_base32(&s).map_err(D::Error::custom)?;
        v.as_slice().try_into().map_err(D::Error::custom)
    } else {
        let v = Vec::<u8>::deserialize(deserializer)?;
        v.as_slice().try_into().map_err(D::Error::custom)
    }
}