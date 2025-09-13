use std::str::FromStr;

use crate::SingleId;


pub fn is_delimiter(c: char) -> bool {
    match c {
        '-' | '_' => true,
        _ => false,
    }
}