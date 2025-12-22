use caretta_sync::{subcommand::DeviceCommandArgs, types::AppInfo, util::RunnableCommand};
use caretta_sync_example_core::APP_NAME;
use clap::{Parser, Subcommand};
use rmcp::model::Implementation;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

impl RunnableCommand for Cli {
    fn run(self, app_info: caretta_sync::types::AppInfo) {
        self.command.run(app_info)
    }
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    Device(DeviceCommandArgs),
}

impl RunnableCommand for CliCommand {
    fn run(self, app_info: caretta_sync::types::AppInfo) {
        match self {
            CliCommand::Device(x) => x.run(app_info),
        }
    }
}
fn main() {
    Cli::parse().run(AppInfo {
        app_name: APP_NAME,
        client_info: Implementation {
            name: "caretta-sync-example-cli".to_string(),
            title: None,
            version: "0.0.0".to_string(),
            icons: None,
            website_url: None,
        },
    })
}
