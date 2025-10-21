use clap::Args;
use iroh::PublicKey;
use mtid::Dtid;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierArgs {
    device_id: Option<Dtid>,
    public_key: Option<PublicKey>,
}
