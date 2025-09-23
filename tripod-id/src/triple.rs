use crate::{utils::is_delimiter, Double, Error, Single};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::TripodId;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triple {
    inner: (Single, Single, Single)
}

impl TripodId for Triple{
    type SizeType = u64;
    const SIZE: Self::SizeType = (Single::SIZE as u64).pow(3);
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
    fn is_valid(&self) -> bool {
        self.inner.0.is_valid() && self.inner.1.is_valid() && self.inner.2.is_valid() && (u64::from(*self) < Self::SIZE)
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
        if value < Self::SIZE {
            Ok(Self{
                inner: (
                    Single::try_from(u16::try_from(value / (Double::SIZE as u64)).unwrap())?,
                    Single::try_from(u16::try_from((value % (Double::SIZE as u64)) /(Single::SIZE as u64)).unwrap())?,
                    Single::try_from(u16::try_from(value % (Single::SIZE as u64)).unwrap())?
                )})
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::SIZE as u64,
                found: value as u64
            })
        }
    }
}

impl From<Triple> for u64 {
    fn from(value: Triple) -> Self {
        (u16::from(value.inner.0) as u64) * (Double::SIZE as u64)
        + (u16::from(value.inner.1) as u64) * (Single::SIZE as u64)
        + (u16::from(value.inner.2) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_random<R>(rand: &mut  R)
    where
        R: Rng
    {
        let id: Triple = rand.r#gen();
        assert!(id.is_valid());
        assert_eq!(id, Triple::from_str(&id.to_string()).unwrap());
        assert_eq!(id, Triple::try_from(u64::from(id)).unwrap());
    }
    #[test]
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}