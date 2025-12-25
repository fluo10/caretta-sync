#[cfg(feature = "macros")]
pub use caretta_framework_macros::Mergeable;
pub trait Mergeable<T = Self>: Sized {
    fn merge(&mut self, other: T);
}
