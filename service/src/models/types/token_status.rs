use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Clone, Debug, EnumIter, DeriveActiveEnum, PartialEq, Eq)]
#[sea_orm(rs_type = "u8", db_type = "Integer")]
pub enum TokenStatus {
    Pending = 0,
    Used = 1,
    Revoked = 2,
    Expired = 3,
}

impl Default for TokenStatus {
    fn default() -> Self {
        Self::Pending
    }
}