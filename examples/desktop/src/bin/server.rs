use caretta_sync::{parser::ServerParser, util::RunnableCommand as _};
use caretta_sync_example_core::APP_NAME;
use clap::Parser;

fn main() {
    let parser = ServerParser::<caretta_sync_example_core::mcp::Service, caretta_sync_example_core::migration::Migrator>::parse();
    parser.run(APP_NAME)    
}
