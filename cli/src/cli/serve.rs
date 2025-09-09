use std::marker::PhantomData;

use clap::Args;
use caretta_sync_core::{config::Config, global::{CONFIG, LOCAL_DATABASE_CONNECTION}, server::ServerTrait, utils::runnable::Runnable};

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServeCommandArgs<T> 
where
    T: ServerTrait
{
    #[arg(skip)]
    server: PhantomData<T>,
    #[command(flatten)]
    config: ConfigArgs,
}
impl<T> Runnable for ServeCommandArgs<T>
where 
    T: ServerTrait
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = CONFIG.get_or_init::<Config>(self.config.into_config(app_name).await).await;
        let _ = LOCAL_DATABASE_CONNECTION.get_or_init(&config.storage.get_local_database_path() );
        T::serve_all(config).await.unwrap();
    }
}