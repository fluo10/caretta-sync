mod generated {
    include!(concat!(env!("OUT_DIR"), "/fireturtle.tripod_id.rs"));
}

mod single;
mod double;
mod triple;

pub use generated::*;

use crate::TripodId;
const PACKAGE_NAME: &'static str = "fireturtle.tripod_id";
pub type SingleMessage = Single;
pub type DoubleMessage = Double;
pub type TripleMessage = Triple;

pub trait TripodIdMessage: From<Self::TripodId> {
    type TripodId: TripodId + TryFrom<Self>;

    fn is_valid(self) -> bool {
        Self::TripodId::try_from(self).is_ok()
    }
}