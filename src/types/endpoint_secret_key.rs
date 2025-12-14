use std::{array::TryFromSliceError, sync::Arc};

crate::types::macros::def_iroh_secret_key!{
    Self = EndpointSecretKey,
    Inner = iroh_base::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError
}

impl_iroh_secret_key!{
    Self = EndpointSecretKey,
    Inner = iroh_base::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError,
    new = iroh_base::SecretKey::generate
}