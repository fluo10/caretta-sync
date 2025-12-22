use crate::types::EndpointPublicKey;

crate::types::macros::def_iroh_secret_key! {
    Self = EndpointSecretKey,
    Inner = iroh_base::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError
}

impl_iroh_secret_key! {
    Self = EndpointSecretKey,
    Inner = iroh_base::SecretKey,
    TryIntoError = TryIntoEndpointSecretKeyError,
    PublicKey = EndpointPublicKey,
    new = iroh_base::SecretKey::generate,
    public_key = iroh_base::SecretKey::public
}
