use prost::Name;

use crate::{prost::Single, Error, TripodId};

impl Name for Single {
    const NAME: &'static str = "Single";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl Single {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        use crate::TripodId;

        self.id < u32::from(crate::Single::SIZE)
    }
}

impl From<crate::Single> for Single {
    fn from(value: crate::Single) -> Self {
        Self {
            id: u32::from(u16::from(value)) 
        }
    }
}
impl TryFrom<Single> for crate::Single {
    type Error = Error;

    fn try_from(value: Single) -> Result<Self, Self::Error> {
        Self::try_from(
            u16::try_from(value.id).or(Err(Error::OutsideOfRange {
                expected: u64::from(crate::Single::SIZE),
                found: u64::from(value.id) 
            }))?
        )
    }
}
