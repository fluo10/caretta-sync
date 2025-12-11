use std::marker::PhantomData;

use caretta_sync_core::{context::ServerContext, util::RunnableCommand};
use caretta_sync_service::server::ServerTrait;
use clap::Parser;

use crate::{
    args::ConfigArgs,
    parsed_config::ParsedConfig,
    types::Verbosity,
};

#[derive(Parser, Debug)]
pub struct ServerParser<S>
where
    S: ServerTrait,
{
    #[arg(skip)]
    server: PhantomData<S>,
    #[command(flatten)]
    config: ConfigArgs,
    #[arg(short, long, value_name = "VERBOSITY")]
    check_config: Option<Option<Verbosity>>
}
impl<S> RunnableCommand for ServerParser<S>
where
    S: ServerTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
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
            verbosity =  Verbosity::Default;
        }
        let config = config
            .with_default(app_name)
            .with_local_database();
        if check_config {
            if verbosity == Verbosity::Verbose {
                let _ = config_to_print.insert(config.clone());
            }
        } else {
            config.init_tracing_subscriber();
        }
        let context = config
            .into_server_context(app_name)
            .await
            .unwrap();
        if check_config {
            if let Some(x) = config_to_print {
                println!("{}", x);
            }
        } else {
            S::serve(context).await.unwrap();
        }
        
    }
}
