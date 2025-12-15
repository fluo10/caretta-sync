use crate::types::NamespacePublicKey;

crate::types::macros::def_iroh_secret_key!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret,
    TryIntoError = TryIntoNamespaceSecretKeyError
}

impl_iroh_secret_key!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret,
    TryIntoError = TryIntoNamespaceSecretKeyError,
    PublicKey = NamespacePublicKey,
    new = iroh_docs::NamespaceSecret::new,
    public_key = iroh_docs::NamespaceSecret::public_key
}
