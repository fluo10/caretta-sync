crate::types::macros::iroh_key_def! {
    SecretKey = (NamespaceSecretKey, iroh_docs::NamespaceSecret),
    PublicKey = (NamespacePublicKey, iroh_docs::NamespacePublicKey)
}

crate::types::macros::iroh_key_impl! {
    SecretKey = (NamespaceSecretKey, iroh_docs::NamespaceSecret),
    PublicKey = (NamespacePublicKey, iroh_docs::NamespacePublicKey)
}