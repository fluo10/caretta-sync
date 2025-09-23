use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{utils::is_delimiter, Error, TripodId, Single};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Double{
    inner: (Single, Single)
}

impl TripodId for Double{
    type SizeType = u32;
    const SIZE: Self::SizeType = (Single::SIZE as u32).pow(2);
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
    fn is_valid(&self) -> bool {
        self.inner.0.is_valid() && self.inner.1.is_valid() && (u32::from(*self) < Self::SIZE)
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
        if value < Self::SIZE {
            Ok(Self{
                inner: (
                    Single::try_from(u16::try_from(value/(Single::SIZE as u32)).unwrap())?,
                    Single::try_from(u16::try_from(value % (Single::SIZE as u32)).unwrap())?
                )})
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::SIZE as u64,
                found: value as u64
            })
        }
    }
}

impl From<Double> for u32 {
    fn from(value: Double) -> Self {
        u32::from(u16::from(value.inner.0)) * u32::from(Single::SIZE) + u32::from(u16::from(value.inner.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_random<R>(rand: &mut  R)
    where
        R: Rng
    {
        let id: Double = rand.r#gen();
        assert!(id.is_valid());
        assert_eq!(id,Double::from_str(&id.to_string()).unwrap());
        assert_eq!(id, Double::try_from(u32::from(id)).unwrap())
    }
    #[test]
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}