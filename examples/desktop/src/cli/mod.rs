use caretta_sync_example_core::{gui::Gui, server::Server};
use clap::{Parser, Subcommand};
use caretta_sync::{cli::*, config::Config, data::migration::DataMigrator, global::{CONFIG, DATABASE_CONNECTIONS}, utils::Runnable};


#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<CliCommand>,
    #[command(flatten)]
    config: ConfigArgs,
}

impl Runnable for Cli {
    fn run(self, app_name: &'static str) {
        if let Some(x) = self.command {
            x.run(app_name)
        } else {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let config: caretta_sync::config::Config = self.config.into_config(app_name).await;
                    let _ = CONFIG.get_or_init::<Config>(config).await;
                });
            //let _ = DATABASE_CONNECTIONS.get_or_init_unchecked(&config, DataMigrator).await;
            Gui{}.run(app_name)
        }
    }
}

#[derive(Debug, Subcommand, Runnable)]
pub enum CliCommand {
    Config(ConfigCommandArgs),
    Device(DeviceCommandArgs),
    Peer(PeerCommandArgs),
    Serve(ServeCommandArgs<Server>),
}