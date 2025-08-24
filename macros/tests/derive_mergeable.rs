use std::collections::{HashMap, HashSet};

use caretta_sync_core::utils::mergeable::Mergeable;
use caretta_sync_macros::Mergeable;

#[derive(Clone, Debug, PartialEq, Mergeable)]
struct MergeableStruct {
    opt: Option<u8>,
}

#[cfg(test)]
fn test() {
    let zero = MergeableStruct{
        opt: Some(0),
    };
    let one = MergeableStruct {
        opt: Some(1),
    };
    let none = MergeableStruct{
        opt: None,
    };
    let mut zero_with_one = zero.clone();
    zero_with_one.merge(one.clone());
    let mut none_with_zero = none.clone();
    none_with_zero.merge(zero.clone());
    let mut zero_with_none = zero.clone();
    zero_with_none.merge(none.clone());
    assert_eq!(zero_with_one.clone(), one.clone());
    assert_eq!(none_with_zero, zero.clone());
    assert_eq!(zero_with_none, zero.clone());
    
}
