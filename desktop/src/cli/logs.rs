use caretta_core::utils::runnable::Runnable;
use clap::Args;

#[derive(Args, Debug)]
pub struct LogsCommandArgs {
    #[arg(short='n', long)]
    lines: Option<u32>,
}

impl Runnable for LogsCommandArgs {
     async fn run(self, app_name: &'static str) {
        todo!()
    }
}