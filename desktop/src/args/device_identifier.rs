use clap::Args;
use mtid::Dtid;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierArgs {
    #[arg(long)]
    id: Option<Dtid>,
    #[arg(long)]
    public_key: Option<String>,
    #[arg(long)]
    name: Option<String>
}

impl From<DeviceIdentifierArgs> for caretta_sync_core::proto::api::device::Identifier {
    fn from(value: DeviceIdentifierArgs) -> Self {
        use caretta_sync_core::proto::api::device::identifier::Value;
        Self {
            value: Some(
                match (value.id, value.public_key, value.name) {
                    (Some(x), None, None) => Value::Id(x.into()),
                    (None, Some(x), None) => Value::PublicKey(base32::decode(base32::Alphabet::Crockford, &x).unwrap()),
                    (None, None, Some(x)) => Value::Name(x),
                    (_, _, _) => unreachable!("The parsed argument must be one."),
                },
            ),
        }
    }
}
