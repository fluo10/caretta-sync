use caretta_sync_core::{ipc::DeviceIdentifier, util::{decode_base32, encode_base32}};
use clap::Args;
use caretta_id::CarettaId;
use iroh::PublicKey;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierArgs {
    #[arg(long)]
    id: Option<CarettaId>,
    #[arg(long)]
    public_key: Option<String>,
    #[arg(long)]
    name: Option<String>,
}

impl From<DeviceIdentifierArgs> for DeviceIdentifier{
    fn from(value: DeviceIdentifierArgs) -> Self {
        match (value.id, value.public_key, value.name) {
            (Some(x), None, None) => Self::Id(x),
            (None, Some(x), None) => Self::PublicKey(PublicKey::from_bytes(decode_base32(&x).unwrap().as_slice().try_into().unwrap()).unwrap()),
            (None, None, Some(x)) => Self::Name(x),
            (_, _, _) => unreachable!("The parsed argument must be one."),
        }
    }
}
