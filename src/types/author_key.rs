super::macros::iroh_key_def! {
    SecretKey = (AuthorSecretKey, iroh_docs::Author),
    PublicKey = (AuthorPublicKey, iroh_docs::AuthorPublicKey)
}


super::macros::iroh_key_impl!{
    SecretKey = ( AuthorSecretKey, iroh_docs::Author),
    PublicKey = ( AuthorPublicKey, iroh_docs::AuthorPublicKey)
}
