use prost::Name;

use crate::{prost::{Single, TripodIdMessage}, Error, TripodId};

impl Name for Single {
    const NAME: &'static str = "Single";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl TripodIdMessage for Single {
    type TripodId = crate::Single;

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
                expected: u64::from(crate::Single::CAPACITY),
                found: u64::from(value.id) 
            }))?
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Single, SingleMessage, TripodId};

    #[test]
    fn nil() {
        let nil = SingleMessage{id: 0};
        assert_eq!(Single::NIL, Single::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = SingleMessage{id: u32::from(Single::CAPACITY)-1};
        assert_eq!(Single::MAX, Single::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized () {
        let oversized = SingleMessage{id: u32::from(Single::CAPACITY)};
        let _ = Single::try_from(oversized).unwrap();
    }
}