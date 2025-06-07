use clap::{Parser, Subcommand};
use lazy_supplements::{cli::{ConfigArgs, ConsoleArgs, ConsoleCommands, InitArgs, NodeArgs, NodeCommand, ServerArgs}, global::{Global, GLOBAL}, *};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[command(flatten)]
    pub config: ConfigArgs,
}

#[derive(Debug, Subcommand)]
enum Command {
    Console(ConsoleArgs),
    Node(NodeArgs),
    Init(InitArgs),
    Server(ServerArgs),
}


#[tokio::main]
async fn main() {
    let cli =  Cli::parse();
    let _ = GLOBAL.get_or_init_node_config(cli.config.try_into_node_config().await.unwrap()).await;
    match cli.command {
        Command::Node(x) => x.run().await.unwrap(),
        Command::Init(x) => x.init_config().await,
        Command::Server(x) => x.start_server().await.unwrap(),
        Command::Console(x) => x.start_console(ConsoleCommands::default()).await.unwrap(),
    }
}