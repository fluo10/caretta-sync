use clap::Args;
use iroh::PublicKey;
use mtid::Dtid;

/// Specify target authorization_request
#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct VerificationIdentifierArgs {
    #[arg(short, long)]
    request_id: Option<Dtid>,
    #[arg(short, long)]
    device_id: Option<Dtid>,
    #[arg(short, long)]
    public_key: Option<PublicKey>,
}

impl From<VerificationIdentifierArgs> for caretta_sync_core::proto::api::device_verification::Identifier {
    fn from(value: VerificationIdentifierArgs) -> Self {
        use caretta_sync_core::proto::api::device_verification::identifier::Value;
        Self{
            value: Some( match (value.request_id, value.device_id, value.public_key) {
                (Some(x), None, None) => Value::VerificationId(x.into()),
                (None, Some(x), None) => Value::DeviceId(x.into()),
                (None, None, Some(x)) => Value::PublicKey(x.into()),
                (_, _, _) => unreachable!("The parsed argument must be one.")
            })
        }
    }
}