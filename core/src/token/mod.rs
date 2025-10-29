mod error;

pub use error::InvitationTokenDeserializeError;
use chrono::{DateTime, Duration, SubsecRound, Utc};
use iroh::{Endpoint, EndpointId};
use mtid::Dtid;

#[derive(Clone, Debug, PartialEq)]
pub struct InvitationToken{
    endpoint_id: EndpointId,
    token_id: Dtid,
    expires_at: DateTime<Utc>
}

impl InvitationToken {
    pub const LENGTH: usize = Self::ENDPOINT_LENGTH + Self::TOKEN_ID_LENGTH + Self::EXPIRES_AT_LENGTH;
    const ENDPOINT_ID_START: usize = 0;
    const ENDPOINT_LENGTH: usize = EndpointId::LENGTH;
    const ENDPOINT_ID_END: usize = Self::ENDPOINT_ID_START + EndpointId::LENGTH;
    const TOKEN_ID_START: usize = Self::ENDPOINT_ID_END;
    const TOKEN_ID_LENGTH: usize = ((u32::BITS/8) as usize);
    const TOKEN_ID_END: usize = Self::TOKEN_ID_START + Self::TOKEN_ID_LENGTH;
    const EXPIRES_AT_START: usize = Self::TOKEN_ID_END;
    const EXPIRES_AT_LENGTH: usize = (i64::BITS / 8) as usize;
    const EXPIRES_AT_END: usize = Self::EXPIRES_AT_START + Self::EXPIRES_AT_LENGTH;

    pub fn new(endpoint_id: EndpointId, model: crate::models::invitation_token::Model) -> Self {
        Self {
            endpoint_id: endpoint_id,
            token_id: model.public_id,
            expires_at: model.expires_at.to_utc()
        }
    }
    pub fn to_bytes(&self) -> [u8;Self::LENGTH] {
        let mut buf = [0u8; Self::LENGTH];
        buf[Self::ENDPOINT_ID_START..Self::ENDPOINT_ID_END].copy_from_slice(self.endpoint_id.as_bytes());
        buf[Self::TOKEN_ID_START..Self::TOKEN_ID_END].copy_from_slice(&u32::from(self.token_id).to_be_bytes());
        buf[Self::EXPIRES_AT_START..Self::EXPIRES_AT_END].copy_from_slice(&self.expires_at.timestamp().to_be_bytes());
        
        buf
    }

    pub fn from_bytes(bytes: [u8;Self::LENGTH]) -> Result<Self, InvitationTokenDeserializeError> {
        let endpoint_id = EndpointId::from_bytes(&bytes[Self::ENDPOINT_ID_START..Self::ENDPOINT_ID_END].try_into().unwrap())?;
        let token_id = u32::from_be_bytes(bytes[Self::TOKEN_ID_START..Self::TOKEN_ID_END].try_into().unwrap()).try_into()?;
        let expires_at = DateTime::from_timestamp(
            i64::from_be_bytes(bytes[Self::EXPIRES_AT_START..Self::EXPIRES_AT_END].try_into().unwrap()),
            0
        ).ok_or(InvitationTokenDeserializeError::DateTimeInvalid)?;
        
        Ok(Self { endpoint_id, token_id, expires_at })
    }
}

#[cfg(test)]
mod tests {
    use iroh::SecretKey;
    use mtid::Dtid;

    use super::*;

    #[test]
    fn bytes_conversion () {
        
        let payload = InvitationToken{
            endpoint_id: SecretKey::generate(&mut rand::rng()).public(),
            token_id: Dtid::random(),
            expires_at : Utc::now().round_subsecs(0),
        };
        let bytes = payload.to_bytes();
        assert_eq!(payload, InvitationToken::from_bytes(bytes).unwrap());
    }
}