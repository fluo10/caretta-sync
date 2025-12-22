use caretta_sync::{parser::ServerParser, types::AppInfo, util::RunnableCommand as _};
use caretta_sync_example_core::APP_NAME;
use clap::Parser;
use rmcp::model::Implementation;

fn main() {
    let parser = ServerParser::<
        caretta_sync_example_core::mcp::Service,
        caretta_sync_example_core::migration::Migrator,
    >::parse();
    parser.run(AppInfo {
        app_name: APP_NAME,
        #[cfg(feature = "client")]
        client_info: Implementation {
            name: "caretta-sync-example-server".to_string(),
            title: None,
            version: "0.0.0".to_string(),
            icons: None,
            website_url: None,
        },
    })
}
