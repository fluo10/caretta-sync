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
//!     #[serde(with = "date_time")]
//!     timestamp: DateTime<Utc>
//! }
//! 
//! # fn main() -> anyhow::Result<()> {
//! let timestamp: DateTime<Utc> = NaiveDate::from_ymd_opt(2025, 11, 21)
//!     .unwrap()
//!     .and_hms_nano_opt(05,58,05, 920_822_380)
//!     .unwrap()
//!     .and_utc();
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

use std::time::Duration;

use chrono::{DateTime, Local, TimeZone};
use serde::{de::{Expected, Unexpected, Error as _}, {ser::{Error as _}}, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S, Tz> (
    value: &DateTime<Tz>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where 
    S: Serializer,
    Tz: TimeZone
{
    if serializer.is_human_readable() {
        value.serialize(serializer)
    } else {
        let secs = u64::try_from(value.timestamp()).map_err(|e| S::Error::custom(e))?;
        let nanos = value.timestamp_subsec_nanos();
        let duration = Duration::new(secs, nanos);
        duration.serialize(serializer)
    }
}

pub fn deserialize<'de, D, Tz>(deserializer: D) -> Result<DateTime<Tz>, D::Error>
where 
    D: Deserializer<'de>,
    DateTime<Tz>: From<DateTime<Local>> + Deserialize<'de>, 
    Tz: TimeZone
{
    if deserializer.is_human_readable() {
        DateTime::<Tz>::deserialize(deserializer)
    } else {
        let duration = Duration::deserialize(deserializer)?;
        let secs = i64::try_from(duration.as_secs()).map_err(|_| D::Error::invalid_value(Unexpected::Unsigned(duration.as_secs()), &"Value that fit within i64"))?;
        let nanos = duration.subsec_nanos();
        Local.timestamp_opt(secs, nanos).single().ok_or(D::Error::invalid_value(Unexpected::Unsigned(nanos as u64), &"Value under 1_000_000_000" )).map(|x| DateTime::<Tz>::from(x))
    }
}
#[cfg(test)]
pub mod tests{
    use chrono::{DateTime, Local};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Test {
        #[serde(with = "super")]
        timestamp: DateTime<Local>,
    }
    #[test]
    fn toml() {
        let time = Local::now();
        let test = Test { timestamp: time};
        let as_string = toml::to_string(&test).unwrap();
        let from_string: Test = toml::from_str(&as_string).unwrap();
        assert_eq!(test, from_string);
    }

    #[test]
    fn cbor () {
        let test = Test { timestamp: Local::now()};
        let mut as_bytes = Vec::new();
        ciborium::into_writer(&test, &mut as_bytes).unwrap();
        let from_bytes: Test = ciborium::from_reader(as_bytes.as_slice()).unwrap();
        assert_eq!(test, from_bytes);

    }
}