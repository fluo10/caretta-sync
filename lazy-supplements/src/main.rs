use clap::{Parser, Subcommand};
use lazy_supplements::{cli::{InitArgs, NodeArgs, NodeCommand, ServerArgs}, *};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Node(NodeArgs),
    Init(InitArgs),
    Server(ServerArgs),
}


#[tokio::main]
async fn main() {
    match Cli::parse().command {
        Command::Node(x) => match x.command {
            NodeCommand::Ping(y) => y.ping().await.unwrap(),
            NodeCommand::Join(y) => println!("{y:?}"),
        },
        Command::Init(x) => x.init_config().await,
        Command::Server(x) => x.start_server().await.unwrap(),
    }
}