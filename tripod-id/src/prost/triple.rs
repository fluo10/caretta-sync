use prost::Name;

use crate::{prost::{Triple, TripodIdMessage}, Error};

impl Name for Triple {
    const NAME: &'static str = "Triple";
    const PACKAGE: &'static str = super::PACKAGE_NAME;
}

impl TripodIdMessage for Triple{
    type TripodId = crate::Triple;
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

#[cfg(test)]
mod tests {
    use crate::{Triple, TripleMessage, TripodId};

    #[test]
    fn nil() {
        let nil = TripleMessage{id: 0};
        assert_eq!(Triple::NIL, Triple::try_from(nil).unwrap());
    }

    #[test]
    fn max() {
        let max = TripleMessage{id: u64::from(Triple::CAPACITY)-1};
        assert_eq!(Triple::MAX, Triple::try_from(max).unwrap());
    }

    #[test]
    #[should_panic]
    fn oversized () {
        let oversized = TripleMessage{id: u64::from(Triple::CAPACITY)};
        let _ = Triple::try_from(oversized).unwrap();
    }
}