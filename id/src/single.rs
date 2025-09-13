use std::{fmt::Display, str::FromStr};

use rand::{distributions::{uniform::UniformInt, Standard}, prelude::Distribution, Rng};

use crate::{error::Error, Id};

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
            expected: CUBED_BASE as usize,
            found: int as usize
        })
    }
    let first_char = char::from(CHARACTERS[usize::try_from(int / SQUARED_BASE).unwrap()]);
    let second_char = char::from(CHARACTERS[usize::try_from((int % SQUARED_BASE)/ BASE).unwrap()]);
    let third_char = char::from(CHARACTERS[usize::try_from(int % BASE).unwrap()]);
    Ok(format!("{}{}{}", first_char, second_char, third_char))
}

#[derive(Clone, Debug, PartialEq)]
pub struct SingleId{
    inner: u16
}


impl Id for SingleId {
    type SizeType = u16;
    const SIZE: Self::SizeType = CUBED_BASE;

    /// ```
    /// use caretta_id::{SingleId, Id};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(SingleId::NIL, SingleId::from_str("000").unwrap());
    /// assert_eq!(SingleId::NIL, SingleId::try_from(0).unwrap());
    /// ```
    const NIL: SingleId = SingleId{
        inner: 0
    };

    /// ```
    /// use caretta_id::{Id, SingleId};
    /// use std::str::FromStr;
    /// 
    /// assert_eq!(SingleId::MAX, SingleId::from_str("zzz").unwrap());
    /// assert_eq!(SingleId::MAX, SingleId::try_from(35936).unwrap());
    /// ```
    const MAX: SingleId = SingleId{
        inner: Self::SIZE-1
    };

    #[cfg(test)]
    fn is_valid(&self) -> bool {
        self.inner < Self::SIZE 
    }
}

impl Display for SingleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u16_to_string(self.inner).unwrap())
    }
}

impl FromStr for SingleId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{
            inner: str_to_u16(s)?
        })
    }
}

impl Distribution<SingleId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SingleId {
        SingleId {
            inner: rng.gen_range(0..SingleId::SIZE)
        }
    }
}

impl TryFrom<u16> for SingleId {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < Self::SIZE {
            Ok(Self{inner: value})
        } else {
            Err(Error::OutsideOfRange{
                expected: Self::SIZE as usize,
                found: value as usize
            })
        }
    }
}

impl From<SingleId> for u16 {
    fn from(value: SingleId) -> Self {
        value.inner
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
        assert!(chunk.is_valid());
        let s = chunk.to_string();
        assert_eq!(chunk,SingleId::from_str(&s).unwrap());
        let i = u16::from(chunk.clone());
        assert_eq!(chunk, SingleId::try_from(i).unwrap());
    }
    #[test]
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}