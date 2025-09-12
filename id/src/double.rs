use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{Error, Id, SingleId};

#[derive(Debug, Clone, PartialEq)]
pub struct DoubleId{
    inner: (SingleId, SingleId)
}

impl DoubleId {
    #[cfg(test)]
    pub fn validate(&self) -> bool {
        self.inner.0.validate() && self.inner.1.validate() && (u32::from(self.clone()) < Self::SIZE)
    }
}


impl Id for DoubleId{
    type SizeType = u32;
    const SIZE: Self::SizeType = (SingleId::SIZE as u32).pow(2);
    /// ```
    /// use caretta_id::{Id, DoubleId};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(DoubleId::NIL, DoubleId::from_str("000-000").unwrap());
    /// assert_eq!(DoubleId::NIL, DoubleId::try_from(0).unwrap());
    /// ```
    const NIL: Self = Self{
        inner: (SingleId::NIL, SingleId::NIL)
    };

    /// ```
    /// use caretta_id::{Id, DoubleId};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(DoubleId::MAX, DoubleId::from_str("zzz-zzz").unwrap());
    /// assert_eq!(DoubleId::MAX, DoubleId::try_from(1291467968).unwrap());
    /// ```
    const MAX: Self = Self{
        inner: (SingleId::MAX, SingleId::MAX) 
    };
}

impl Display for DoubleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.inner.0, self.inner.1)
    }
}

impl FromStr for DoubleId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner : match s.len() {
                7 => {
                    match s.chars().nth(3).unwrap() {
                        '-' =>  {
                            Ok((SingleId::from_str(&s[0..3])?, SingleId::from_str(&s[4..7])?))
                        }, 
                        x => {
                            Err(Error::InvalidDelimiter(x))
                        }
                    }
                    
                }
                6 => {
                    Ok((SingleId::from_str(&s[0..3])?,SingleId::from_str(&s[3..6])?))
                }
                _ => Err(Error::InvalidLength(s.to_string()))
            }?
        })
    }
}


impl Distribution<DoubleId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DoubleId {
        DoubleId {
            inner: (rng.r#gen(), rng.r#gen())
        }
    }
}

impl TryFrom<u32> for DoubleId {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < Self::SIZE {
            Ok(Self{
                inner: (
                    SingleId::try_from(u16::try_from(value/(SingleId::SIZE as u32)).unwrap())?,
                    SingleId::try_from(u16::try_from(value % (SingleId::SIZE as u32)).unwrap())?
                )})
        } else {
            Err(Error::OutsideOfRange(value as u64))
        }
    }
}

impl From<DoubleId> for u32 {
    fn from(value: DoubleId) -> Self {
        u32::from(u16::from(value.inner.0)) * u32::from(SingleId::SIZE) + u32::from(u16::from(value.inner.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_random<R>(rand: &mut  R)
    where
        R: Rng
    {
        let chunk: SingleId = rand.r#gen();
        let s = chunk.to_string();
        assert_eq!(chunk,SingleId::from_str(&s).unwrap())
    }
    #[test]
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}