use crate::{Error, SingleId};

pub struct TripleId {
    inner: (SingleId, SingleId, SingleId)
}

