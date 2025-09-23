    {
        include!(concat!(env!("OUT_DIR"), "/fireturtle.tripod_id.rs"));
    }
    pub use crate::fireturtle::tripod_id::*;
    type SingleMessage = Single;
    type DoubleMessage = Double;
    type TripleMessage = Triple;