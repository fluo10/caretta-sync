pub fn encode_base32(value: &[u8]) -> String {
    base32::encode(base32::Alphabet::Crockford, value).to_ascii_lowercase()
}

pub fn decode_base32(value: &str) -> Result<Vec<u8>, DecodeBase32Error> {
    base32::decode(base32::Alphabet::Crockford, value).ok_or(DecodeBase32Error(value.to_string()))
}

#[derive(Debug, thiserror::Error)]
#[error("Decode base32 error: {0}")]
pub struct DecodeBase32Error(pub String);