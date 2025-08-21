#[cfg(feature="macros")]
pub use caretta_macros::Mergeable;
pub trait Mergeable: Sized {
    fn merge(&mut self, other: Self);
}