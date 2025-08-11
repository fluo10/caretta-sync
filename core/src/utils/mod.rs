use prost_types::Timestamp;
use chrono::{DateTime, Timelike, Utc};
pub mod async_convert;
pub mod emptiable;
pub mod mergeable;
pub mod runnable;

pub fn utc_to_timestamp(utc: DateTime<Utc>) -> Timestamp {
    Timestamp{
        seconds: utc.timestamp(),
        nanos: i32::try_from(utc.nanosecond()).unwrap(),
    }
}