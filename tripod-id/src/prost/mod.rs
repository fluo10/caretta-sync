mod generated {
    include!(concat!(env!("OUT_DIR"), "/fireturtle.tripod_id.rs"));
}

mod single;
mod double;
mod triple;

pub use generated::*;
const PACKAGE_NAME: &'static str = "fireturtle.tripod_id";
pub type SingleMessage = Single;
pub type DoubleMessage = Double;
pub type TripleMessage = Triple;

pub trait TrypodIdMessage: From<Self::TrypodId> {
    type TrypodId: crate::TripodId + TryFrom<Self>;

    #[cfg(test)]
    fn is_valid(&self) -> bool;
}