use crate::types::NamespaceSecretKey;

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
    SecretKey = NamespaceSecretKey
}