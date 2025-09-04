use std::time::Duration;

use iroh::endpoint::Source;

use crate::{error::Error, proto::SourceMessage};

impl TryFrom<(Source, Duration)> for SourceMessage {
    type Error = Error;
    fn try_from(src: (Source, Duration)) -> Result<Self, Self::Error> {
        let (source, duration )= src;
        Ok(Self {
            source: source.to_string(),
            duration: Some(duration.try_into()?),
        })
    }
} 