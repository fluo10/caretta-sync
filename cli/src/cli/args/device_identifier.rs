use clap::Args;
use iroh::PublicKey;
use iroh_tickets::endpoint::EndpointTicket;
use mtid::Dtid;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierArgs {
    #[arg(long)]
    device_id: Option<Dtid>,
    #[arg(long)]
    endpoint_id: Option<PublicKey>,
    #[arg(long)]
    endpoint_ticket: Option<EndpointTicket>,
}

impl From<DeviceIdentifierArgs> for caretta_sync_core::proto::api::device::Identifier {
    fn from(value: DeviceIdentifierArgs) -> Self {
        use caretta_sync_core::proto::api::device::identifier::Value;
        Self {
            value: Some(
                match (value.device_id, value.endpoint_id, value.endpoint_ticket) {
                    (Some(x), None, None) => Value::Id(x.into()),
                    (None, Some(x), None) => Value::EndpointId(x.into()),
                    (None, None, Some(x)) => Value::EndpointTicket(x.into()),
                    (_, _, _) => unreachable!("The parsed argument must be one."),
                },
            ),
        }
    }
}
