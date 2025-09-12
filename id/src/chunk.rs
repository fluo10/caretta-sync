use std::{fmt::Display, str::FromStr};

use rand::Rng;

use crate::error::Error;

static CHARACTERS: &[u8;33] = b"0123456789abcdefghjkmnpqrstuvwxyz";

static BASE: u16 = 33;
static SQUARED_BASE: u16 = 1089;
static CUBED_BASE: u16 = 35937;

pub static NIL: IdChunk = IdChunk(0);
pub static MAX: IdChunk = IdChunk(CUBED_BASE-1);

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
        return Err(Error::OutsideOfRange(int))
    }
    let first_char = char::from(CHARACTERS[usize::try_from(int / SQUARED_BASE).unwrap()]);
    let second_char = char::from(CHARACTERS[usize::try_from((int % SQUARED_BASE)/ BASE).unwrap()]);
    let third_char = char::from(CHARACTERS[usize::try_from(int % BASE).unwrap()]);
    Ok(format!("{}{}{}", first_char, second_char, third_char))
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdChunk(u16);

impl IdChunk {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self(rng.gen_range(0..CUBED_BASE))

    }
}

impl Display for IdChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for IdChunk {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl TryFrom<u16> for IdChunk {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<IdChunk> for u16 {
    fn from(value: IdChunk) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_eq_chunk(s: &str, i: u16) {
        assert_eq!(s, &u16_to_string(i).unwrap());
        assert_eq!(i, str_to_u16(s).unwrap())

    }
    #[test]
    fn test_nil() {
        assert_eq_chunk("000", 0);
    }
    #[test]
    fn test_max() {
        assert_eq_chunk("zzz", CUBED_BASE-1);
    }
    fn assert_random<R>(rand: &mut  R)
    where
        R: Rng
    {
        let chunk = IdChunk::new(rand);
        let s = chunk.to_string();
        assert_eq!(chunk,IdChunk::from_str(&s).unwrap())
    }
    fn random_x10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            assert_random(&mut rng);
        }

    }
}