use std::str::FromStr;

use crate::Single;


pub fn is_delimiter(c: char) -> bool {
    match c {
        '-' | '_' => true,
        _ => false,
    }
}