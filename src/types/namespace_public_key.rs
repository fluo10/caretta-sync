use std::{array::TryFromSliceError, sync::Arc};

use iroh::KeyParsingError;
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::util::DecodeBase32Error;

crate::types::macros::def_iroh_public_key!{
    Self = NamespacePublicKey,
    Inner = iroh_docs::NamespacePublicKey,
    TryIntoError = TryIntoNamespacePublicKeyError,
    InvalidBytesValueInner = ed25519_dalek::SignatureError
}

impl_iroh_public_key!{
    Self = NamespacePublicKey,
    Inner = iroh_docs::NamespacePublicKey,
    TryIntoError = TryIntoNamespacePublicKeyError,
}