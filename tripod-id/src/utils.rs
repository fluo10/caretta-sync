/// Test if the character is valid delimiter.
pub fn is_delimiter(c: char) -> bool {
    match c {
        '-' | '_' => true,
        _ => false,
    }
}