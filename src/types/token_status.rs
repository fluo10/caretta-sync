use std::{fmt::Display, str::FromStr};

use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::Serialize;

#[derive(Copy, Clone, Debug, EnumIter, DeriveActiveEnum, PartialEq, Eq)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum TokenStatus {
    Pending = 0,
    Used = 1,
    Revoked = 2,
    Expired = 3,
}

impl TokenStatus {
    const PENDING_INT: u8 = 0;
    const PENDING_STR: &str = "pending";
    const USED_INT: u8 = 1;
    const USED_STR: &str = "used";
    const REVOKED_INT: u8 = 2;
    const REVOKED_STR: &str  = "revoked";
    const EXPIRED_INT: u8 = 3;
    const EXPIRED_STR: &str = "expired";
}

impl From<TokenStatus> for u8 {
    fn from(value: TokenStatus) -> Self {
        match value {
            TokenStatus::Pending => TokenStatus::PENDING_INT,
            TokenStatus::Used => TokenStatus::USED_INT,
            TokenStatus::Revoked => TokenStatus::REVOKED_INT,
            TokenStatus::Expired => TokenStatus::EXPIRED_INT
        }
    }
}

impl TryFrom<u8> for TokenStatus {
    type Error = TryIntoTokenStatusError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        
        match value {
            TokenStatus::PENDING_INT => Ok(TokenStatus::Pending),
            TokenStatus::USED_INT => Ok(TokenStatus::Used),
            TokenStatus::REVOKED_INT => Ok(TokenStatus::Revoked),
            TokenStatus::EXPIRED_INT => Ok(TokenStatus::Expired),
            i => Err(TryIntoTokenStatusError::InvalidInt(i))
        }
    }
}

impl Display for TokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TokenStatus::Pending => TokenStatus::PENDING_STR,
            TokenStatus::Used => TokenStatus::USED_STR,
            TokenStatus::Revoked => TokenStatus::REVOKED_STR,
            TokenStatus::Expired => TokenStatus::EXPIRED_STR
        })
    }
}

impl FromStr for TokenStatus {
    type Err = TryIntoTokenStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            TokenStatus::PENDING_STR => Ok(TokenStatus::Pending),
            TokenStatus::USED_STR => Ok(TokenStatus::Used),
            TokenStatus::REVOKED_STR => Ok(TokenStatus::Revoked),
            TokenStatus::EXPIRED_STR => Ok(TokenStatus::Expired),
            s => Err(TryIntoTokenStatusError::InvalidStr(s.to_string()))
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
pub enum TryIntoTokenStatusError {
    #[error("Invalid str: {0}")]
    InvalidStr(String),
    #[error("Invalid int: {0}")]
    InvalidInt(u8),
}
