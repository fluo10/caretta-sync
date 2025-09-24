mod single;
mod double;
mod error;
mod triple;
mod utils;
#[cfg(feature="rusqlite")]
mod rusqlite;
#[cfg(feature="serde")]
mod serde;

use std::{fmt::Display, str::FromStr};

pub use single::*;
pub use double::*;
pub use triple::*;
pub use error::*;

#[cfg(feature="prost")]
pub mod prost;
#[cfg(feature="prost")]
pub use prost::{ SingleMessage, DoubleMessage, TripleMessage };

pub trait TripodId: Copy + Display + TryFrom<Self::Integer, Error=Error> + FromStr<Err=Error> + PartialEq {
    type Integer: From<Self>;
    const NIL: Self;
    const MAX: Self;
    const CAPACITY: Self::Integer;
    #[cfg(test)]
    fn validate_inner(self) -> bool;

    #[cfg(test)]
    fn validate_string_convertion(self) -> Result<bool, Error> {
        Ok(self == Self::from_str(&self.to_string())?)
    }

    #[cfg(test)]
    fn validate_integer_conversion(self) -> Result<bool, Error> {
        Ok(self == Self::try_from(Self::Integer::from(self))?)
    }

    #[cfg(test)]
    fn validate_all(self) -> Result<bool, Error> {
        Ok(self.validate_inner() 
            && self.validate_string_convertion()? 
            && self.validate_integer_conversion()?
        )
    }
}
#[cfg(test)]
mod tests {
    use std::{fmt::Display, fmt::Debug, str::FromStr};

    #[cfg(feature="prost")]
use crate::prost::TrypodIdMessage;

    use super::*;

    #[cfg(feature="prost")]
    pub fn assert_valid_message<T, M>(id: &T) where 
    T: TripodId + Debug + Display + FromStr<Err=Error> + PartialEq + TryFrom<M, Error = Error> + Copy,
    M: TrypodIdMessage<TrypodId = T> + From<T>
    {
        let message = M::from(*id);
        assert!(message.is_valid());
        assert_eq!(*id, T::try_from(message).unwrap());
    }

    pub fn assert_id_eq_int<T, I> (id: T, int: I ) where 
    T: TripodId<Integer = I> + Debug + PartialEq + TryFrom<I, Error = Error> + Copy,
    I: From<T> + PartialEq + Debug + Copy
    {
        assert_eq!(id, T::try_from(int).unwrap());
        assert_eq!(I::from(id), int);
    }

    pub fn assert_id_eq_str<T> (id: T, code: &str ) where 
    T: TripodId + Debug + Display + FromStr<Err=Error> + PartialEq + Copy,
    {
        assert_eq!(id, T::from_str(code).unwrap());
    }

}