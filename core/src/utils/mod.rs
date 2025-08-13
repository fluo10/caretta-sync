use prost_types::Timestamp;
use chrono::{DateTime, TimeZone, Timelike, Utc};
pub mod async_convert;
pub mod emptiable;
pub mod mergeable;
pub mod runnable;

/// ## Examples
/// ```
/// use chrono::Utc;
/// use std::time::SystemTime;
/// use prost_types::Timestamp;
/// use caretta_core::utils::utc_to_timestamp;
/// 
/// let now_utc = Utc::now();
/// let now_timestamp = utc_to_timestamp(&now_utc);
/// assert_eq!(SystemTime::try_from(now_utc).unwrap(), SystemTime::try_from(now_timestamp).unwrap());
/// ```
pub fn utc_to_timestamp(utc: &DateTime<Utc>) -> Timestamp {
    Timestamp{
        seconds: utc.timestamp(),
        nanos: i32::try_from(utc.nanosecond()).unwrap(),
    }
}

/// ## Examples
/// ```
/// use std::time::SystemTime;
/// use prost_types::Timestamp;
/// use caretta_core::utils::timestamp_to_utc;
///  
/// let now_timestamp = Timestamp::from(SystemTime::now());
/// let now_utc = timestamp_to_utc(&now_timestamp);
/// assert_eq!(SystemTime::try_from(now_utc).unwrap(), SystemTime::try_from(now_timestamp).unwrap());
/// ```
pub fn timestamp_to_utc(t: &Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(t.seconds, u32::try_from(t.nanos).unwrap()).unwrap()
}
