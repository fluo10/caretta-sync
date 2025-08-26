use caretta_sync::utils::Runnable;
use caretta_sync_example_core::global::APP_NAME;
use clap::Parser;

use crate::cli::Cli;

fn main() {
    let args = Cli::parse();
    args.run(APP_NAME);
}
