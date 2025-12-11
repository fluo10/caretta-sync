use crate::{error::CoreError, types::{EndpointPublicKey, TryIntoEndpointPublicKeyError}};
use chrono::{DateTime, Duration, Local, SubsecRound, Utc};
use iroh::{ EndpointId};
use sea_orm::{DatabaseConnection, DbErr};
use caretta_id::CarettaId;

#[derive(Clone, Debug, PartialEq)]
pub struct InvitationToken {
    token_id: CarettaId,
    secret: i64,
    endpoint_id: EndpointPublicKey,
}

impl InvitationToken {
    pub const LENGTH: usize =
        Self::SECRET_LENGTH + Self::ENDPOINT_LENGTH + Self::TOKEN_ID_LENGTH;
    
    const TOKEN_ID_START: usize = 0;
    const TOKEN_ID_LENGTH: usize = 5;
    const TOKEN_ID_END: usize = Self::TOKEN_ID_START + Self::TOKEN_ID_LENGTH;
    const SECRET_START: usize = Self::TOKEN_ID_END;
    const SECRET_LENGTH: usize = 8;
    const SECRET_END: usize = Self::SECRET_START + Self::SECRET_LENGTH;
    const ENDPOINT_ID_START: usize = Self::SECRET_END;
    const ENDPOINT_LENGTH: usize = EndpointPublicKey::LENGTH;
    const ENDPOINT_ID_END: usize = Self::ENDPOINT_ID_START + EndpointPublicKey::LENGTH;

    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut buf = [0u8; Self::LENGTH];
        buf[Self::TOKEN_ID_START..Self::TOKEN_ID_END]
            .copy_from_slice(&self.token_id.to_be_bytes_compact());
        buf[Self::SECRET_START..Self::SECRET_END]
            .copy_from_slice(&self.secret.to_be_bytes());
        buf[Self::ENDPOINT_ID_START..Self::ENDPOINT_ID_END]
            .copy_from_slice(self.endpoint_id.as_bytes());
        buf
    }

    pub fn from_bytes(bytes: [u8; Self::LENGTH]) -> Result<Self, TryIntoInvitationTokenError> {
        let token_id = CarettaId::from_be_bytes_compact_lossy(
            bytes[Self::TOKEN_ID_START..Self::TOKEN_ID_END]
            .try_into()
                .unwrap(),
        );
        let secret = i64::from_be_bytes(
            bytes[Self::SECRET_START..Self::SECRET_END]
                .try_into()
                .unwrap()
        );
        let endpoint_id = EndpointPublicKey::from_bytes(
            &bytes[Self::ENDPOINT_ID_START..Self::ENDPOINT_ID_END]
                .try_into()
                .unwrap(),
        )?;
        
        Ok(Self {
            secret,
            token_id,
            endpoint_id,
        })
    }
    
    pub async fn new<T>(context: &T) -> Result<Self, DbErr> 
    where 
    T: AsRef<DatabaseConnection>
    {
        todo!()
    }
}


#[derive(Debug, thiserror::Error)]
pub enum TryIntoInvitationTokenError {
    #[error(transparent)]
    InvalidEndpointId(#[from] TryIntoEndpointPublicKeyError),
    #[error("Invalid token id: {0}")]
    TokenIdOversized(#[from] caretta_id::Error),
    #[error("Invalid date time value.")]
    DateTimeInvalid,
    
}


#[cfg(test)]
mod tests {
    use iroh::SecretKey;
    use caretta_id::CarettaId;

    use super::*;

    #[test]
    fn bytes_conversion() {
        let payload = InvitationToken {
            secret: rand::random(),
            endpoint_id: EndpointPublicKey::from(SecretKey::generate(&mut rand::rng()).public()),
            token_id: CarettaId::random(),
        };
        let bytes = payload.to_bytes();
        assert_eq!(payload, InvitationToken::from_bytes(bytes).unwrap());
    }
}
