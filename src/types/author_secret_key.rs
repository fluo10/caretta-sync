use std::{array::TryFromSliceError, sync::Arc};

use sea_orm::{
    DbErr, TryGetError, TryGetable, Value,
    sea_query::{Nullable, ValueType, ValueTypeErr},
};
use serde::{Deserialize, Serialize};

use crate::util::DecodeBase32Error;

super::macros::def_iroh_secret_key!{
    Self = AuthorSecretKey,
    Inner = iroh_docs::Author,
    TryIntoError = TryIntoAuthorSecretKeyError
}

super::macros::impl_iroh_secret_key!{
    Self = AuthorSecretKey,
    Inner = iroh_docs::Author,
    TryIntoError = TryIntoAuthorSecretKeyError,
    new = iroh_docs::Author::new
}

