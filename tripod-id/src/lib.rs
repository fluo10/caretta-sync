mod single;
mod double;
mod error;
mod triple;
mod utils;
#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="serde")]
mod serde;

pub use single::*;
pub use double::*;
pub use triple::*;
pub use error::*;

#[cfg(feature="prost")]
pub mod prost;

pub trait TripodId {
    type SizeType;
    const NIL: Self;
    const MAX: Self;
    const SIZE: Self::SizeType;
    #[cfg(test)]
    fn is_valid(&self) -> bool;
}

