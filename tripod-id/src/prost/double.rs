use prost::Name;

use crate::{prost::Double, Error, TripodId};

impl Name for Double {
    const NAME: &'static str = "Double";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl Double {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        use crate::TripodId;

        self.id < u32::from(crate::Double::CAPACITY)
    }
}

impl From<crate::Double> for Double {
    fn from(value: crate::Double) -> Self {
        Self {
            id: u32::from(value) 
        }
    }
}
impl TryFrom<Double> for crate::Double {
    type Error = Error;

    fn try_from(value: Double) -> Result<Self, Self::Error> {
        Self::try_from(
            value.id
        )
    }
}