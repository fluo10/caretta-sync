use crate::{utils::is_delimiter, DoubleId, Error, SingleId};

use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::Id;

#[derive(Debug, Clone, PartialEq)]
pub struct TripleId {
    inner: (SingleId, SingleId, SingleId)
}

impl Id for TripleId{
    type SizeType = u64;
    const SIZE: Self::SizeType = (SingleId::SIZE as u64).pow(3);
    /// ```
    /// use caretta_id::{Id, TripleId};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(TripleId::NIL, TripleId::from_str("000-000-000").unwrap());
    /// assert_eq!(TripleId::NIL, TripleId::try_from(0).unwrap());
    /// ```
    const NIL: Self = Self{
        inner: (SingleId::NIL, SingleId::NIL, SingleId::NIL)
    };

    /// ```
    /// use caretta_id::{Id, TripleId};
    /// use std::str::FromStr;
    ///
    /// assert_eq!(TripleId::MAX, TripleId::from_str("zzz-zzz-zzz").unwrap());
    /// assert_eq!(TripleId::MAX, TripleId::try_from(46411484401952).unwrap());
    /// ```
    const MAX: Self = Self{
        inner: (SingleId::MAX, SingleId::MAX, SingleId::MAX) 
    };

    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.inner.0.is_valid() && self.inner.1.is_valid() && self.inner.2.is_valid() && (u64::from(self.clone()) < Self::SIZE)
    }
}

impl Display for TripleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.inner.0, self.inner.1, self.inner.2)
    }
}

impl FromStr for TripleId {
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
                        Ok((SingleId::from_str(&s[0..3])?,SingleId::from_str(&s[4..7])?,SingleId::from_str(&s[8..11])?))
                    } else {
                        Err(Error::InvalidDelimiter{
                            found: Vec::from(delimiter),
                            raw: s.to_string()
                        })
                    }

                }
                9 => {
                    Ok((SingleId::from_str(&s[0..3])?,SingleId::from_str(&s[3..6])?,SingleId::from_str(&s[6..9])?))
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


impl Distribution<TripleId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TripleId {
        TripleId {
            inner: (rng.r#gen(), rng.r#gen(), rng.r#gen())
        }
    }
}

impl TryFrom<u64> for TripleId {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < Self::SIZE {
            Ok(Self{
                inner: (
                    SingleId::try_from(u16::try_from(value / (DoubleId::SIZE as u64)).unwrap())?,
                    SingleId::try_from(u16::try_from((value % (DoubleId::SIZE as u64)) /(SingleId::SIZE as u64)).unwrap())?,
                    SingleId::try_from(u16::try_from(value % (SingleId::SIZE as u64)).unwrap())?
                )})
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::SIZE as usize,
                found: value as usize
            })
        }
    }
}

impl From<TripleId> for u64 {
    fn from(value: TripleId) -> Self {
        (u16::from(value.inner.0) as u64) * (DoubleId::SIZE as u64)
        + (u16::from(value.inner.1) as u64) * (SingleId::SIZE as u64)
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
        let id: TripleId = rand.r#gen();
        assert!(id.is_valid());
        assert_eq!(id, TripleId::from_str(&id.to_string()).unwrap());
        assert_eq!(id, TripleId::try_from(u64::from(id.clone())).unwrap());
    }
    #[test]
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}