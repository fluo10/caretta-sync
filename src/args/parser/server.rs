use std::marker::PhantomData;

use crate::{config::ServerConfig, mcp::ServiceContext, types::AppInfo, util::RunnableCommand};
use clap::Parser;
use rmcp::{RoleServer, Service};
use sea_orm_migration::MigratorTrait;

use crate::{args::option::ConfigOptionArgs, config::parsed::ParsedConfig, types::Verbosity};

#[derive(Parser, Debug)]
pub struct ServerParser<S, M>
where
    S: Service<RoleServer> + From<&'static ServiceContext> + Send + 'static,
    M: MigratorTrait,
{
    #[arg(skip)]
    server: PhantomData<S>,
    #[arg(skip)]
    migrator: PhantomData<M>,
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[arg(short, long, value_name = "VERBOSITY")]
    check_config: Option<Option<Verbosity>>,
}
impl<S, M> RunnableCommand for ServerParser<S, M>
where
    S: Service<RoleServer> + From<&'static ServiceContext> + Send + 'static,
    M: MigratorTrait,
{
    #[tokio::main]
    async fn run(self, app_info: AppInfo) {
        let app_name = app_info.name;
        let mut check_config: bool;
        let mut verbosity: Verbosity;
        let mut config_to_print: Option<ParsedConfig> = None;
        let config = self.config.into_parsed_config(app_name);
        if let Some(x) = self.check_config.as_ref() {
            check_config = true;
            verbosity = x.clone().unwrap_or(Verbosity::default());
            if verbosity == Verbosity::Default {
                let _ = config_to_print.insert(config.clone());
            }
        } else {
            check_config = false;
            verbosity = Verbosity::Default;
        }
        let config = config.with_default(app_name).with_database().await;
        if check_config {
            if verbosity == Verbosity::Verbose {
                let _ = config_to_print.insert(config.clone());
            }
        } else {
            config.init_tracing_subscriber();
        }
        let config = config.into_server_config(app_name).unwrap();
        if check_config {
            if let Some(x) = config_to_print {
                println!("{}", x);
            }
        } else {
            config.spawn_server::<S, M>(app_name).await
        }
    }
}
