use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[cfg(feature="prost")]
use crate::DoubleMessage;
use crate::{utils::is_delimiter, Error, TripodId, Single};

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Double(u32);

impl TripodId for Double{
    type Tuple = (Single, Single);
    type Integer = u32;
    #[cfg(feature="prost")]
    type Message = DoubleMessage;
    const CAPACITY: Self::Integer = (Single::CAPACITY as u32).pow(2);

    const NIL: Self = Self(0);

    const MAX: Self = Self(Self::CAPACITY -1);

    #[cfg(test)]
    fn validate_inner(self) -> bool {
        self.0 < Self::CAPACITY
    }
}

impl Display for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuple: (Single, Single) = (*self).into();
        write!(f, "{}-{}", tuple.0, tuple.1)
    }
}

impl From<(Single, Single)> for Double {
    fn from(value: (Single, Single)) -> Self {
        Self(u32::from(u16::from(value.0)) * u32::from(Single::CAPACITY) + u32::from(u16::from(value.1)))
    }
} 

impl From<Double> for (Single, Single) {
    fn from(value: Double) -> Self {
        (
            Single::try_from(u16::try_from(value.0/(Single::CAPACITY as u32)).unwrap()).unwrap(),
            Single::try_from(u16::try_from(value.0 % (Single::CAPACITY as u32)).unwrap()).unwrap()
        )
    }
}

impl FromStr for Double {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tuple = match s.len() {
            7 => {
                let delimiter = s[3..4].chars().next().unwrap();
                if is_delimiter(delimiter) {
                    Ok((Single::from_str(&s[0..3])?,Single::from_str(&s[4..7])?))
                } else {
                    Err(Error::InvalidDelimiter{
                        found: vec![delimiter],
                        raw: s.to_string()
                    })
                }
                
            },
            6 => {
                Ok((Single::from_str(&s[0..3])?,Single::from_str(&s[3..6])?))
            },
            x => { 
                Err(Error::InvalidLength{
                    expected: (6, 7),
                    found: x,
                    raw: s.to_string()
                })
            }
        }?;
        Ok(Self::from(tuple))
    }
}


impl Distribution<Double> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Double {
        Double(rng.gen_range(0..Double::CAPACITY))

    }
}

impl TryFrom<u32> for Double {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
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

impl From<Double> for u32 {
    fn from(value: Double) -> Self {
        value.0
    }
}


impl PartialEq<u32> for Double {
    fn eq(&self, other: &u32) -> bool {
        &u32::from(*self) == other
    }
}

impl PartialEq<String> for Double {
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
        assert!(Double::NIL.validate_all().unwrap());
        assert_eq!(Double::NIL, 0);
        assert_eq!(Double::NIL, "000000".to_string());
        assert_eq!(Double::NIL, "000-000".to_string());
        assert!(Double::NIL.is_nil());
        assert!(!Double::NIL.is_max());

    }

    #[test]
    fn max() {
        assert!(Double::MAX.validate_all().unwrap());
        assert_eq!(Double::MAX, Double::CAPACITY-1);
        assert_eq!(Double::MAX, "zzzzzz".to_string());
        assert_eq!(Double::MAX, "ZZZ-ZZZ".to_string());
        assert!(Double::MAX.is_max());
        assert!(!Double::MAX.is_nil());
    }

    #[test]
    #[should_panic]
    fn over_sized() {
        Double::try_from(Double::CAPACITY).unwrap();
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let id: Double = rng.r#gen();
            assert!(id.validate_all().unwrap());
        }
    }
}