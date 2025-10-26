use clap::Args;
use iroh::PublicKey;
use iroh_tickets::endpoint::EndpointTicket;
use mtid::Dtid;

use crate::cli::DeviceIdentifierArgs;

/// Specify target authorization_request
#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct VerificationIdentifierArgs {
    #[arg(short, long)]
    verificaton_id: Option<Dtid>,
    #[command(flatten)]
    device: DeviceIdentifierArgs,
}

impl From<VerificationIdentifierArgs> for caretta_sync_core::proto::api::device_verification::Identifier {
    fn from(value: VerificationIdentifierArgs) -> Self {
        use caretta_sync_core::proto::api::device_verification::identifier::Value;
        Self{
            value: Some( match (value.verificaton_id, value.device) {
                (Some(x), _) => Value::VerificationId(x.into()),
                (None, x) => Value::Device(x.into()),
                (_, _) => unreachable!("The parsed argument must be one.")
            })
        }
    }
}