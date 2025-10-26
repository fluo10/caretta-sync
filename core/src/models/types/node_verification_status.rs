use sea_orm::{DeriveActiveEnum, EnumIter};

use crate::proto::{api::device_verification::list_request::Status, ProtoDeserializeError};

#[derive(Copy, Clone, Debug, EnumIter, DeriveActiveEnum, PartialEq, Eq)]
#[sea_orm(rs_type = "u16", db_type = "SmallUnsigned")]
pub enum NodeVerificationStatus {
    Pending = 1,
    Approved = 2,
    Rejected = 3,
    Expired = 4,
    Canceled = 5,
}

impl TryFrom<Status> for NodeVerificationStatus {
    type Error = ProtoDeserializeError;
    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value {
            Status::Pending => Ok(Self::Pending),
            Status::Unspecified => Err(ProtoDeserializeError::EnumUnspecified("caretta_sync.api.device_verification.ListRequest.Status")),
            Status::Approved => Ok(Self::Approved),
            Status::Rejected => Ok(Self::Rejected),
            Status::Expired => Ok(Self::Expired),
            Status::Canceled => Ok(Self::Canceled),
        }
    }
}

