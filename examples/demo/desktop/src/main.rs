use caretta_sync::utils::runnable::Runnable;
use caretta_sync_demo_core::global::APP_NAME;
use clap::Parser;

use crate::cli::Cli;

mod cli;
mod ipc;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    args.run(APP_NAME).await;
}
