super::macros::iroh_public_key_def! { AuthorPublicKey(iroh_docs::AuthorPublicKey)}


super::macros::iroh_public_key_impl!{
    Self = AuthorPublicKey,
    Inner = iroh_docs::AuthorPublicKey,
    SecretKey = crate::types::AuthorSecretKey
}