use std::{fmt::Display, str::FromStr};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[cfg(feature="prost")]
use crate::SingleMessage;
use crate::{error::Error, TripodId};

const CHARACTERS: &[u8;33] = b"0123456789abcdefghjkmnpqrstuvwxyz";
const BASE: u16 = 33;
const SQUARED_BASE: u16 = BASE.pow(2);
const CUBED_BASE: u16 = BASE.pow(3);

fn char_to_u8(c: char) -> Option<u8> {
    Some(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        'g' => 16,
        'h' => 17,
        'i' => 1,
        'j' => 18,
        'k' => 19,
        'l' => 1,
        'm' => 20,
        'n' => 21,
        'o' => 0,
        'p' => 22,
        'q' => 23,
        'r' => 24,
        's' => 25,
        't' => 26,
        'u' => 27,
        'v' => 28,
        'w' => 29,
        'x' => 30,
        'y' => 31,
        'z' => 32,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 1,
        'J' => 18,
        'K' => 19,
        'L' => 1,
        'M' => 20,
        'N' => 21,
        'O' => 0,
        'P' => 22,
        'Q' => 23,
        'R' => 24,
        'S' => 25,
        'T' => 26,
        'U' => 27,
        'V' => 28,
        'W' => 29,
        'X' => 30,
        'Y' => 31,
        'Z' => 32,
        _ => return None 
    })
}

fn str_to_u16(s: &str) -> Result<u16, Error> {
    if s.len() != 3 {
        return Err(Error::InvalidChunk(format!("Chunk '{}' is not 3 characters", s)))
    }
    let mut buf : [u16;3] = [0;3];
    for (i, c) in s.chars().enumerate() {
        buf[i] = BASE.pow((2 - i) as u32) * (char_to_u8(c).ok_or(Error::InvalidChunk(format!("Invalid char: {}", c)))? as u16);
    }

    Ok(buf.iter().sum())
}
fn u16_to_string(int: u16) -> Result<String, Error> {
    if int >= CUBED_BASE {
        return Err(Error::OutsideOfRange{
            expected: CUBED_BASE as u64,
            found: int as u64
        })
    }
    let first_char = char::from(CHARACTERS[usize::try_from(int / SQUARED_BASE).unwrap()]);
    let second_char = char::from(CHARACTERS[usize::try_from((int % SQUARED_BASE)/ BASE).unwrap()]);
    let third_char = char::from(CHARACTERS[usize::try_from(int % BASE).unwrap()]);
    Ok(format!("{}{}{}", first_char, second_char, third_char))
}

/// Single size tripod id.
/// 
/// # Examples
/// 
/// ```
/// use std::str::FromStr;
/// use tripod_id::{TripodId,Single};
/// 
/// assert_eq!(Single::from_str("012").unwrap(), Single::try_from(35).unwrap());
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct Single(u16);


impl TripodId for Single {
    type Integer = u16;
    type Tuple = (Single,);
    #[cfg(feature="prost")]
    type Message = SingleMessage;

    const CAPACITY: Self::Integer = CUBED_BASE;

    const NIL: Single = Single(0);

    const MAX: Single = Single(Self::CAPACITY-1);

    #[cfg(test)]
    fn validate_inner(self) -> bool {
        self.0 < Self::CAPACITY
    }
}

impl Display for Single {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u16_to_string(self.0).unwrap())
    }
}

impl FromStr for Single {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(str_to_u16(s)?))
    }
}

impl Distribution<Single> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Single {
        Single(rng.gen_range(0..Single::CAPACITY))
    }
}

impl TryFrom<u16> for Single {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
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

impl From<Single> for u16 {
    fn from(value: Single) -> Self {
        value.0
    }
}

impl From<(Single,)> for Single {
    fn from(value: (Single,)) -> Self {
        value.0
    }
}
impl From<Single> for (Single,) {
    fn from(value: Single) -> Self {
        (value,)
    }
}

impl PartialEq<u16> for Single {
    fn eq(&self, other: &u16) -> bool {
        &u16::from(*self) == other
    }
}

impl PartialEq<String> for Single {
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
        assert!(Single::NIL.validate_all().unwrap());
        assert_eq!(Single::NIL, 0);
        assert!(Single::NIL.validate_parse_strings(&["000"]).unwrap());
        assert!(Single::NIL.is_nil());
        assert!(!Single::NIL.is_max())
    }

    #[test]
    fn max() {
        assert!(Single::MAX.validate_all().unwrap());
        assert_eq!(Single::MAX, Single::CAPACITY - 1);
        assert!(Single::MAX.validate_parse_strings(&["zzz", "ZZZ"]).unwrap());
        assert!(Single::MAX.is_max());
        assert!(!Single::MAX.is_nil());
    }

    #[test]
    #[should_panic]
    fn over_sized() {
        Single::try_from(Single::CAPACITY).unwrap();
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let single: Single = rng.r#gen();
            assert!(single.validate_all().unwrap());
        }
    }


}