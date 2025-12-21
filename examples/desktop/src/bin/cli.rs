use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    
}
fn main() {
    todo!()
}