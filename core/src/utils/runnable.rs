#[cfg(feature="macros")]
pub use caretta_macros::Runnable;

pub trait Runnable {
    async fn run(self, app_name: &'static str);
}