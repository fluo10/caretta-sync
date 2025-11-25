use caretta_sync_core::types::Base32Bytes;
use clap::Args;
use caretta_id::CarettaId;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct TokenIdentifierArgs {
    #[arg(long)]
    id: Option<CarettaId>,
    #[arg(long)]
    token: Option<Base32Bytes>,
}

impl From<TokenIdentifierArgs> for caretta_sync_core::proto::api::invitation_token::Identifier {
    fn from(value: TokenIdentifierArgs) -> Self {
        use caretta_sync_core::proto::api::invitation_token::identifier::Value;
        Self {
            value: Some(match (value.id, value.token) {
                (Some(x), None) => Value::Id(x.into()),
                (None, Some(x)) => Value::Token(
                    caretta_sync_core::proto::api::invitation_token::InvitationToken {
                        value: x.into(),
                    },
                ),
                (_, _) => unreachable!("The parsed argument must be one."),
            }),
        }
    }
}
