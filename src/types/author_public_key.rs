use std::{array::TryFromSliceError, sync::Arc};

use iroh::{KeyParsingError,};
use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::util::DecodeBase32Error;
super::macros::def_iroh_public_key!{
    Self = AuthorPublicKey,
    Inner = iroh_docs::AuthorPublicKey,
    TryIntoError = TryIntoAuthorIdError,
    InvalidBytesValueInner = ed25519_dalek::SignatureError
}

super::macros::impl_iroh_public_key!{
    Self = AuthorPublicKey,
    Inner = iroh_docs::AuthorPublicKey,
    TryIntoError = TryIntoAuthorIdError,
}


