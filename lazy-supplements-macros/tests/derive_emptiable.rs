use std::collections::{HashMap, HashSet};

use lazy_supplements_core::utils::emptiable::Emptiable;
use lazy_supplements_macros::Emptiable;

#[derive(Debug, PartialEq, Emptiable)]
struct EmptiableStruct{
    vec: Vec<u8>,
    text: String,
    map: HashMap<u8, u8>,
    set: HashSet<u8>,
    opt: Option<u8>,
}

#[cfg(test)]
fn test() {
    use std::hash::Hash;
    let empty = EmptiableStruct::empty();
    assert_eq!(&empty, &EmptiableStruct{
        vec: Vec::new(),
        text: String::new(),
        map: HashMap::new(),
        set: HashSet::new(),
        opt: None,
    })
}
