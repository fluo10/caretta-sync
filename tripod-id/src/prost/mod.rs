mod single;
mod double;
mod triple;

use crate::TripodId;

const PACKAGE_NAME: &'static str = "tripod_id";

include!(concat!(env!("OUT_DIR"), "/tripod_id.rs"));

/// Alias of single tripod-id message
pub type SingleMessage = Single;

/// Alias of double tripod-id message
pub type DoubleMessage = Double;

/// Alias of triple tripod-id message
pub type TripleMessage = Triple;

pub trait TripodIdMessage: From<Self::TripodId> {
    type TripodId: TripodId + TryFrom<Self>;

    fn is_valid(self) -> bool {
        Self::TripodId::try_from(self).is_ok()
    }
}