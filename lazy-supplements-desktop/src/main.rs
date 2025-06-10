use clap::{Parser, Subcommand};
use lazy_supplements_desktop::{cli::{ConfigArgs, NodeArgs, NodeCommand, ServerArgs}, global::{Global, GLOBAL}, *};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    pub config: ConfigArgs,
}

#[derive(Debug, Subcommand)]
enum Command {
    Node(NodeArgs),
    Server(ServerArgs),
}


#[tokio::main]
async fn main() {
    let cli =  Cli::parse();
    let _ = GLOBAL.get_or_init_node_config(cli.config.try_into_node_config().await.unwrap()).await;
    match cli.command {
        Command::Node(x) => x.run().await.unwrap(),
        Command::Server(x) => x.start_server().await.unwrap(),
    }
}