use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

crate::types::macros::def_iroh_secret_key!{
    Self = EndpointSecretKey,
    Inner = iroh::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError
}

impl_iroh_secret_key!{
    Self = EndpointSecretKey,
    Inner = iroh::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError,
    new = iroh::SecretKey::generate
}