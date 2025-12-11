use clap::Args;
use caretta_id::CarettaId;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct TokenIdentifierArgs {
    #[arg(long)]
    id: Option<CarettaId>,
    #[arg(long)]
    token: Option<String>,
}
