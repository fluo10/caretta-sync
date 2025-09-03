use std::time::Duration;

use iroh::endpoint::Source;

use crate::proto::iroh::SourceMessage;

impl TryFrom<(Source, Duration)> for SourceMessage {
    type Error = prost_types::DurationError;
    fn try_from(src: (Source, Duration)) -> Result<Self, Self::Error> {
        let (source, duration )= src;
        Ok(Self {
            source: source.to_string(),
            duration: Some(duration.try_into()?),
        })
    }
} 