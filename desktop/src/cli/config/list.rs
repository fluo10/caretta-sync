use clap::Args;
use crate::utils::runnable::Runnable;
use crate::cli::ConfigArgs;

#[derive(Debug, Args)]
pub struct ConfigListCommandArgs{
    #[command(flatten)]
    config: ConfigArgs,
    #[arg(short,long)]
    all: bool
}

impl Runnable for ConfigListCommandArgs {
    async fn run(self) {
        todo!()
    }
}