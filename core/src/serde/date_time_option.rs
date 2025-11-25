//! Ser/de [`DateTime<Local>`]
//! 
//! Intended for use with serde's with attribute.
//! 
//! # Example
//! ```
//! # use caretta_sync_core::serde::date_time;
//! # use chrono::{DateTime, NaiveDate, Utc};
//! # use serde::{Deserialize, Serialize};
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "date_time_option")]
//!     timestamp: Option<DateTime<Utc>>
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let timestamp: Option<DateTime<Utc>> = Some(NaiveDate::from_ymd_opt(2025, 11, 21)
//!     .unwrap()
//!     .and_hms_nano_opt(05,58,05, 920_822_380)
//!     .unwrap()
//!     .and_utc());
//! let foo = Foo {timestamp};
//! let as_string = toml::to_string(&foo)?;
//! assert_eq!(as_string, "timestamp = \"2025-11-21T05:58:05.920822380Z\"\n");
//! 
//! let from_string = toml::from_str(&as_string)?;
//! assert_eq!(foo, from_string);
//! 
//! let mut as_bytes = Vec::new();
//! ciborium::into_writer(&foo, &mut as_bytes)?;
//! assert_eq!(as_bytes, vec![161, 105, 116, 105, 109, 101, 115, 116, 97, 109, 112, 162, 100, 115, 101, 99, 115, 26, 105, 31, 255, 109, 101, 110, 97, 110, 111, 115, 26, 54, 226, 162, 108]);
//! 
//! let from_bytes: Foo = ciborium::from_reader(as_bytes.as_slice())?;
//! assert_eq!(foo, from_bytes);
//! # Ok(())
//! # }
//! 
//! ```

use std::{fmt, marker::PhantomData, time::Duration};

use chrono::{DateTime, Local, TimeZone};
use serde::{de::{self, Error as _, Expected, Unexpected}, ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};

use crate::serde::date_time;

pub fn serialize<S, Tz> (
    value: &Option<DateTime<Tz>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
    Tz: TimeZone
{
    match value.as_ref() {
        Some(timestamp) => {

            if serializer.is_human_readable() {
                serializer.serialize_some(timestamp)
            } else {
                let milisecs = timestamp.timestamp_millis();
                serializer.serialize_some(&milisecs)
            }
        },
        None => serializer.serialize_none()
    }
}

pub fn deserialize<'de, D, Tz>(deserializer: D) -> Result<Option<DateTime<Tz>>, D::Error>
where 
    D: Deserializer<'de>,
    DateTime<Tz>: From<DateTime<Local>> + Deserialize<'de>, 
    Tz: TimeZone
{
    deserializer.deserialize_option(OptionDateTimeVisitor::<Tz>::new())
}

struct OptionDateTimeVisitor<Tz>(PhantomData<Tz>);

impl<Tz> OptionDateTimeVisitor<Tz> {
    fn new() -> Self {
        Self(PhantomData::<Tz>)
    }
}

impl<'de, Tz> de::Visitor<'de> for OptionDateTimeVisitor<Tz> 
where 
DateTime<Tz>: From<DateTime<Local>> + serde::Deserialize<'de>,
Tz: TimeZone
{
    type Value = Option<DateTime<Tz>>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result{
        formatter.write_str("a byte array or none")
    }

    fn visit_some<D>(self, deserializer:D) -> Result<Self::Value, D::Error>
    where 
        D: Deserializer<'de>,
        {
            Ok(Some(date_time::deserialize(deserializer)?))
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
