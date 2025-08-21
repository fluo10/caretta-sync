use caretta::utils::runnable::Runnable;
use caretta_example_core::global::APP_NAME;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod ipc;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    args.run(APP_NAME).await;
}
