mod single;
mod double;
mod error;
mod triple;

use rand::Rng;
pub use single::*;
pub use double::*;
pub use triple::*;
pub use error::*;


const DOUBLE_ID_SIZE: u32 = (SingleId::SIZE as u32).pow(2);
const TRIPLE_ID_SIZE: u64 = (SingleId::SIZE as u64).pow(3);

pub trait Id {
    type SizeType;
    const NIL: Self;
    const MAX: Self;
    const SIZE: Self::SizeType;
}