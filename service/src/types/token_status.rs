use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize, de::Error as _};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenStatus {
    Pending,
    Used,
    Revoked,
    Expired,
}

impl TokenStatus {
    const PENDING_STR: &str = "pending";
    const USED_STR: &str = "used";
    const REVOKED_STR: &str = "revoked";
    const EXPIRED_STR: &str = "expired";
    const PENDING_INT: u8 = 0;
    const USED_INT: u8 = 1;
    const REVOKED_INT: u8 = 2;
    const EXPIRED_INT: u8 = 3;
}

impl Display for TokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Pending => Self::PENDING_STR,
            Self::Used => Self::USED_STR,
            Self::Expired => Self::EXPIRED_STR,
            Self::Revoked => Self::REVOKED_STR
        })
    }
}

impl FromStr for TokenStatus {
    type Err = TokenStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::PENDING_STR => Ok(Self::Pending),
            Self::USED_STR => Ok(Self::Used),
            Self::EXPIRED_STR => Ok(Self::Expired),
            Self::REVOKED_STR => Ok(Self::Revoked),
            _ => Err(TokenStatusError::InvalidStr(s.to_string()))
        }        
    }
}
impl From<TokenStatus> for u8 {
    fn from(value: TokenStatus) -> Self {
        match value  {
            TokenStatus::Pending => TokenStatus::PENDING_INT,
            TokenStatus::Used => TokenStatus::USED_INT,
            TokenStatus::Revoked => TokenStatus::REVOKED_INT,
            TokenStatus::Expired => TokenStatus::EXPIRED_INT,
        }
    }
}
impl TryFrom<u8> for TokenStatus {
    type Error = TokenStatusError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            TokenStatus::PENDING_INT => Ok(Self::Pending), 
            TokenStatus::USED_INT => Ok(Self::Used),
            TokenStatus::EXPIRED_INT => Ok(Self::Expired),
            TokenStatus::REVOKED_INT => Ok(Self::Revoked),
            x => Err(TokenStatusError::InvalidInt(x))
        }
    }
}

impl<'de> Deserialize<'de> for TokenStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            (&s).parse::<Self>().map_err(D::Error::custom)
        } else {
            let i = u8::deserialize(deserializer)?;
            Self::try_from(i).map_err(D::Error::custom)
        }
    }
}
impl Serialize for TokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_u8((*self).into())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenStatusError {
    #[error("Invaid string: {0}")]
    InvalidStr(String),
    #[error("Invalid value: {0}")]
    InvalidInt(u8),
}