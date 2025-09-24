use crate::{utils::is_delimiter, Double, Error, Single};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::TripodId;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triple {
    inner: (Single, Single, Single)
}

impl TripodId for Triple{
    type Integer = u64;
    const CAPACITY: Self::Integer = (Single::CAPACITY as u64).pow(3);
    /// ```
    /// use tripod_id::{TripodId, Triple};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(Triple::NIL, Triple::from_str("000-000-000").unwrap());
    /// assert_eq!(Triple::NIL, Triple::try_from(0).unwrap());
    /// ```
    const NIL: Self = Self{
        inner: (Single::NIL, Single::NIL, Single::NIL)
    };

    /// ```
    /// use tripod_id::{TripodId, Triple};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Triple::MAX, Triple::from_str("zzz-zzz-zzz").unwrap());
    /// assert_eq!(Triple::MAX, Triple::try_from(46411484401952).unwrap());
    /// ```
    const MAX: Self = Self{
        inner: (Single::MAX, Single::MAX, Single::MAX) 
    };

    #[cfg(test)]
    fn validate_inner(self) -> bool {
        self.inner.0.validate_inner() 
            && self.inner.1.validate_inner()
            && self.inner.2.validate_inner()
            && (u64::from(self) < Self::CAPACITY)
    }
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.inner.0, self.inner.1, self.inner.2)
    }
}

impl FromStr for Triple {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner : match s.len() {
                11 => {
                    let delimiter = [
                        s[3..4].chars().next().unwrap(),
                        s[7..8].chars().next().unwrap(),
                    ];
                    if is_delimiter(delimiter[0]) && is_delimiter(delimiter[1]) {
                        Ok((Single::from_str(&s[0..3])?,Single::from_str(&s[4..7])?,Single::from_str(&s[8..11])?))
                    } else {
                        Err(Error::InvalidDelimiter{
                            found: Vec::from(delimiter),
                            raw: s.to_string()
                        })
                    }

                }
                9 => {
                    Ok((Single::from_str(&s[0..3])?,Single::from_str(&s[3..6])?,Single::from_str(&s[6..9])?))
                }
                x => {
                    Err(Self::Err::InvalidLength{
                        expected: (9, 11),
                        found: x,
                        raw: s.to_string()
                    })
                }
            } ?
        })
    }
}


impl Distribution<Triple> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triple {
        Triple {
            inner: (rng.r#gen(), rng.r#gen(), rng.r#gen())
        }
    }
}

impl TryFrom<u64> for Triple {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::CAPACITY {
            Ok(Self{
                inner: (
                    Single::try_from(u16::try_from(value / (Double::CAPACITY as u64)).unwrap())?,
                    Single::try_from(u16::try_from((value % (Double::CAPACITY as u64)) /(Single::CAPACITY as u64)).unwrap())?,
                    Single::try_from(u16::try_from(value % (Single::CAPACITY as u64)).unwrap())?
                )})
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
        (u16::from(value.inner.0) as u64) * (Double::CAPACITY as u64)
        + (u16::from(value.inner.1) as u64) * (Single::CAPACITY as u64)
        + (u16::from(value.inner.2) as u64)
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
use crate::tests::{assert_id_eq_int, assert_id_eq_str};

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