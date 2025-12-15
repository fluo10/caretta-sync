use std::{array::TryFromSliceError, sync::Arc};

use crate::{types::AuthorSecretKey, util::DecodeBase32Error};
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
    SecretKey = AuthorSecretKey
}


