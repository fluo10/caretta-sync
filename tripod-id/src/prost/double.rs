use prost::Name;

use crate::{prost::{Double, TripodIdMessage}, Error};

impl Name for Double {
    const NAME: &'static str = "Double";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl TripodIdMessage for Double {
    type TripodId = crate::Double;
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

#[cfg(test)]
mod tests {
    use crate::{Double, DoubleMessage, TripodId};

    #[test]
    fn nil() {
        let nil = DoubleMessage{id: 0};
        assert_eq!(Double::NIL, Double::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = DoubleMessage{id: u32::from(Double::CAPACITY)-1};
        assert_eq!(Double::MAX, Double::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized () {
        let oversized = DoubleMessage{id: u32::from(Double::CAPACITY)};
        let _ = Double::try_from(oversized).unwrap();
    }
}