use crate::{
    mcp::model::DeviceIdentifier,
    types::EndpointPublicKey,
    util::{decode_base32, encode_base32},
};
use caretta_id::CarettaId;
use clap::Args;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierOptionArgs {
    #[arg(long)]
    id: Option<CarettaId>,
    #[arg(long)]
    public_key: Option<EndpointPublicKey>,
    #[arg(long)]
    name: Option<String>,
}

impl From<DeviceIdentifierOptionArgs> for DeviceIdentifier {
    fn from(value: DeviceIdentifierOptionArgs) -> Self {
        match (value.id, value.public_key, value.name) {
            (Some(x), None, None) => Self::Id(x),
            (None, Some(x), None) => Self::PublicKey(x),
            (None, None, Some(x)) => Self::Name(x),
            (_, _, _) => unreachable!("The parsed argument must be one."),
        }
    }
}
