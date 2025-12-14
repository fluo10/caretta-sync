crate::types::macros::def_iroh_secret_key!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret,
    TryIntoError = TryIntoNamespaceSecretKeyError
}

impl_iroh_secret_key!{
    Self = NamespaceSecretKey,
    Inner = iroh_docs::NamespaceSecret,
    TryIntoError = TryIntoNamespaceSecretKeyError,
    new = iroh_docs::NamespaceSecret::new
}
