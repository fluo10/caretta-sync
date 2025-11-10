use crate::{
    args::ConfigArgs,
    types::Verbosity,
};
use caretta_sync_core::util::RunnableCommand;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GuiParser
{
    #[command(flatten)]
    config: ConfigArgs,
    #[arg(short, long, value_name = "VERBOSITY")]
    check_config: Option<Option<Verbosity>>,
}
impl RunnableCommand for GuiParser
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        if let Some(x) = self.check_config {
            todo!()
        } else {
            todo!()
        }
    }
}
