crate::types::macros::def_iroh_public_key! {
    Self = EndpointPublicKey,
    Inner = iroh_base::PublicKey,
    TryIntoError = TryIntoEndpointPublicKeyError,
    InvalidBytesValueInner = iroh_base::KeyParsingError
}

impl_iroh_public_key! {
    Self = EndpointPublicKey,
    Inner = iroh_base::PublicKey,
    TryIntoError = TryIntoEndpointPublicKeyError,
    SecretKey = crate::types::EndpointSecretKey
}
