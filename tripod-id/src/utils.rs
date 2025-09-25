use std::str::FromStr;

use crate::Single;

/// Test if the character is valid delimiter.
pub fn is_delimiter(c: char) -> bool {
    match c {
        '-' | '_' => true,
        _ => false,
    }
}