use std::marker::PhantomData;

use caretta_sync_core::{
    config::Config,
    global::CONFIG,
    server::ServerTrait,
    utils::runnable::Runnable,
};
use clap::Args;
use sea_orm_migration::MigratorTrait;

use super::ConfigArgs;

#[derive(Args, Debug)]
pub struct ServeCommandArgs<M, S>
where
    M: MigratorTrait,
    S: ServerTrait,
{
    #[arg(skip)]
    migrator: PhantomData<M>,
    #[arg(skip)]
    server: PhantomData<S>,
    #[command(flatten)]
    config: ConfigArgs,
}
impl<M, S> Runnable for ServeCommandArgs<M, S>
where
    M: MigratorTrait,
    S: ServerTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = CONFIG
            .get_or_init::<Config>(self.config.into_config(app_name).await)
            .await;
        S::serve::<_, M>(config).await.unwrap();
    }
}
