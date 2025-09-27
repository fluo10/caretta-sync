#[cfg(feature="prost")]
use crate::TripleMessage;
use crate::{utils::is_delimiter, Double, Error, Single};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::TripodId;

/// Triple length tripod id.
/// 
/// # Examples 
/// ```
/// # use tripod_id::{TripodId, Triple};
/// # use std::str::FromStr;
/// 
/// let _ = Triple::from_str("012-abc-def");
/// ``` 
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triple(u64);

impl TripodId for Triple{
    type Integer = u64;
    type Tuple = (Single, Single, Single);
    #[cfg(feature="prost")]
    type Message = TripleMessage;
    const CAPACITY: Self::Integer = (Single::CAPACITY as u64).pow(3);

    const NIL: Self = Self(0);

    const MAX: Self = Self(Self::CAPACITY - 1);

    #[cfg(test)]
    fn validate_inner(self) -> bool {
        self.0 < Self::CAPACITY
    }
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let tuple: (Single, Single, Single) = (*self).into();
        write!(f, "{}-{}-{}", tuple.0, tuple.1, tuple.2)
    }
}

impl FromStr for Triple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.len() {
                11 => {
                    let delimiter = [
                        s[3..4].chars().next().unwrap(),
                        s[7..8].chars().next().unwrap(),
                    ];
                    if is_delimiter(delimiter[0]) && is_delimiter(delimiter[1]) {
                        Ok(Self::from((Single::from_str(&s[0..3])?,Single::from_str(&s[4..7])?,Single::from_str(&s[8..11])?)))
                    } else {
                        Err(Error::InvalidDelimiter{
                            found: Vec::from(delimiter),
                            raw: s.to_string()
                        })
                    }

                }
                9 => {
                    Ok(Self::from((Single::from_str(&s[0..3])?,Single::from_str(&s[3..6])?,Single::from_str(&s[6..9])?)))
                }
                x => {
                    Err(Self::Err::InvalidLength{
                        expected: (9, 11),
                        found: x,
                        raw: s.to_string()
                    })
                }
            } ?
        )
    }
}


impl Distribution<Triple> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triple {
        Triple(rng.gen_range(0..Triple::CAPACITY))

    }
}

impl TryFrom<u64> for Triple {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self(value))
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::CAPACITY as u64,
                found: value as u64
            })
        }
    }
}

impl From<Triple> for u64 {
    fn from(value: Triple) -> Self {
        value.0
    }
}

impl From<(Single, Single, Single)> for Triple {
    fn from(value: (Single, Single, Single)) -> Self {
        Self(
            (u16::from(value.0) as u64) * (Double::CAPACITY as u64)
                + (u16::from(value.1) as u64) * (Single::CAPACITY as u64)
                + (u16::from(value.2) as u64)
        )
    }
}

impl From<Triple> for (Single, Single, Single) {
    fn from(value: Triple) -> Self {
        (
            Single::try_from(u16::try_from(value.0 / (Double::CAPACITY as u64)).unwrap()).unwrap(),
            Single::try_from(u16::try_from((value.0 % (Double::CAPACITY as u64)) /(Single::CAPACITY as u64)).unwrap()).unwrap(),
            Single::try_from(u16::try_from(value.0 % (Single::CAPACITY as u64)).unwrap()).unwrap()
        )
    }
}

impl PartialEq<u64> for Triple {
    fn eq(&self, other: &u64) -> bool {
        &u64::from(*self) == other
    }
}

impl PartialEq<String> for Triple {
    fn eq(&self, other: &String) -> bool {
        match Self::from_str(other) {
            Ok(x) => *self == x,
            Err(_) => false
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn nil() {
        assert!(Triple::NIL.validate_all().unwrap());
        assert_eq!(Triple::NIL, 0);
        assert_eq!(Triple::NIL, "000000000".to_string());
        assert_eq!(Triple::NIL, "000-000-000".to_string());

    }

    #[test]
    fn max() {
        assert!(Triple::MAX.validate_all().unwrap());
        assert_eq!(Triple::MAX, Triple::CAPACITY-1);
        assert_eq!(Triple::MAX, "zzzzzzzzz".to_string());
        assert_eq!(Triple::MAX, "ZZZ-ZZZ-ZZZ".to_string());
        assert_eq!((Single::MAX, Single::MAX, Single::MAX), Triple::MAX.into())
    }

    #[test]
    #[should_panic]
    fn over_sized() {
        Triple::try_from(Triple::CAPACITY).unwrap();
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let id: Triple = rng.r#gen();
            assert!(id.validate_all().unwrap());
        }
    }
}