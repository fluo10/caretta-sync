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
#[cfg(feature="prost")]
pub use prost::{ SingleMessage, DoubleMessage, TripleMessage };

pub trait TripodId {
    type SizeType;
    const NIL: Self;
    const MAX: Self;
    const SIZE: Self::SizeType;
    #[cfg(test)]
    fn is_valid(&self) -> bool;
}
#[cfg(test)]
mod tests {
    use std::{fmt::Display, fmt::Debug, str::FromStr};

    use super::*;

    #[cfg(feature="prost")]
    fn assert_prost(id: T) where 
    T: TripodId<SizeType = I> + Debug + Display + FromStr<Err=Error> + PartialEq + TryFrom<I, Error = Error> + Copy,
    I: From<T> {
        use crate::SingleMessage;
        let message = SingleMessage::from(*id);
        assert_eq!(message.is_valid());
        let result = Single::try_from(message).unwrap();
        assert_eq!(id, result);
    }

    fn assert_id<T, I> (id: T) where 
    T: TripodId<SizeType = I> + Debug + Display + FromStr<Err=Error> + PartialEq + TryFrom<I, Error = Error> + Copy,
    I: From<T>
    {
        assert!(id.is_valid());
        let s = id.to_string();
        assert_eq!(id,T::from_str(&s).unwrap());
        let i = T::SizeType::from(id);
        assert_eq!(id, T::try_from(i).unwrap());
    }
}