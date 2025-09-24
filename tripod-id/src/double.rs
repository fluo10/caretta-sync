use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{utils::is_delimiter, Error, TripodId, Single};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Double{
    inner: (Single, Single)
}

impl TripodId for Double{
    type Integer = u32;
    const CAPACITY: Self::Integer = (Single::CAPACITY as u32).pow(2);
    /// ```
    /// use tripod_id::{TripodId, Double};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(Double::NIL, Double::from_str("000-000").unwrap());
    /// assert_eq!(Double::NIL, Double::try_from(0).unwrap());
    /// ```
    const NIL: Self = Self{
        inner: (Single::NIL, Single::NIL)
    };

    /// ```
    /// use tripod_id::{TripodId, Double};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Double::MAX, Double::from_str("zzz-zzz").unwrap());
    /// assert_eq!(Double::MAX, Double::try_from(1291467968).unwrap());
    /// ```
    const MAX: Self = Self{
        inner: (Single::MAX, Single::MAX) 
    };

    #[cfg(test)]
    fn validate_inner(self) -> bool {
        self.inner.0.validate_inner() && self.inner.1.validate_inner() && (u32::from(self) < Self::CAPACITY)
    }
}

impl Display for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.inner.0, self.inner.1)
    }
}

impl FromStr for Double {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner : match s.len() {
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
                    
                }
                6 => {
                    Ok((Single::from_str(&s[0..3])?,Single::from_str(&s[3..6])?))
                }
                x => Err(Error::InvalidLength{
                    expected: (6, 7),
                    found: x,
                    raw: s.to_string()
                })
            }?
        })
    }
}


impl Distribution<Double> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Double {
        Double {
            inner: (rng.r#gen(), rng.r#gen())
        }
    }
}

impl TryFrom<u32> for Double {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self{
                inner: (
                    Single::try_from(u16::try_from(value/(Single::CAPACITY as u32)).unwrap())?,
                    Single::try_from(u16::try_from(value % (Single::CAPACITY as u32)).unwrap())?
                )})
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
        u32::from(u16::from(value.inner.0)) * u32::from(Single::CAPACITY) + u32::from(u16::from(value.inner.1))
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
use crate::tests::{assert_id_eq_int, assert_id_eq_str};

    use super::*;
    #[test]
    fn nil() {
        assert!(Double::NIL.validate_all().unwrap());
        assert_eq!(Double::NIL, 0);
        assert_eq!(Double::NIL, "000000".to_string());
        assert_eq!(Double::NIL, "000-000".to_string());

    }

    #[test]
    fn max() {
        assert!(Double::MAX.validate_all().unwrap());
        assert_eq!(Double::MAX, Double::CAPACITY-1);
        assert_eq!(Double::MAX, "zzzzzz".to_string());
        assert_eq!(Double::MAX, "ZZZ-ZZZ".to_string());
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