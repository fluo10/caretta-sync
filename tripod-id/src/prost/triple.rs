use prost::Name;

use crate::{prost::Triple, Error, TripodId};

impl Name for Triple {
    const NAME: &'static str = "Triple";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl Triple {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        use crate::TripodId;

        self.id < crate::Triple::SIZE
    }
}

impl From<crate::Triple> for Triple {
    fn from(value: crate::Triple) -> Self {
        Self {
            id: u64::from(value)
        }
    }
}
impl TryFrom<Triple> for crate::Triple {
    type Error = Error;

    fn try_from(value: Triple) -> Result<Self, Self::Error> {
        Self::try_from(
            value.id
        )
    }
}