#[cfg(feature="macros")]
pub use caretta_macros::Mergeable;
pub trait Mergeable: Sized {
    fn merge(&mut self, other: Self);
}

impl<T> Mergeable for Option<T> {
    fn merge(&mut self, mut other: Self) {
        match other.take() {
            Some(x) => {
                let _ = self.insert(x);
            },
            None => {}
        };
    }
}