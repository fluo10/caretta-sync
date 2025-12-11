use std::{array::TryFromSliceError, sync::Arc};

use iroh::KeyParsingError;
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::util::DecodeBase32Error;

crate::types::macros::def_iroh_public_key!{
    Self = EndpointPublicKey,
    Inner = iroh::PublicKey,
    TryIntoError = TryIntoEndpointPublicKeyError,
    InvalidBytesValueInner = iroh::KeyParsingError
}

impl_iroh_public_key!{
    Self = EndpointPublicKey,
    Inner = iroh::PublicKey,
    TryIntoError = TryIntoEndpointPublicKeyError,
}