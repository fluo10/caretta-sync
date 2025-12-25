crate::types::macros::iroh_key_def! {
    SecretKey = (EndpointSecretKey, iroh::SecretKey),
    PublicKey = (EndpointPublicKey, iroh::PublicKey)
}
crate::types::macros::iroh_key_impl! {
    SecretKey = (EndpointSecretKey, iroh::SecretKey),
    PublicKey = (EndpointPublicKey, iroh::PublicKey)
}