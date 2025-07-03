#[cfg(feature="macros")]
pub use lazy_supplements_macros::Runnable;

pub trait Runnable {
    async fn run(self);
}