use std::time::Duration;

use iroh::endpoint::ControlMsg;
use prost_types::DurationError;

use crate::proto::iroh::LastControlMessage;

impl TryFrom<(Duration, ControlMsg)> for LastControlMessage {
    type Error = DurationError;
    fn try_from(value: (Duration, ControlMsg)) -> Result<Self, Self::Error> {
        Ok(LastControlMessage {
            duration: Some(value.0.try_into()?),
            control_msg: value.1.to_string()
        })
    }
}