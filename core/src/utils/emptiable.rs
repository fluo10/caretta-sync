use std::collections::{HashMap, HashSet};
#[cfg(feature="macros")]
pub use caretta_sync_macros::Emptiable;

pub trait Emptiable{
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
}

impl<T> Emptiable for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for Option<T> {
    fn empty() -> Self {
        None
    }
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

impl Emptiable for String {
    fn empty() -> Self {
        String::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T, U> Emptiable for HashMap<T, U> {
    fn empty() -> Self {
        HashMap::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Emptiable for HashSet<T> {
    fn empty() -> Self {
        HashSet::new()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}