use std::marker::PhantomData;

use caretta_sync_service::server::ServerTrait;
use caretta_sync_core::{ context::ServerContext, util::RunnableCommand};
use clap::Args;
use sea_orm_migration::MigratorTrait;

use crate::{args::ConfigArgs};

#[derive(Args, Debug)]
pub struct ServerArgs<M, S>
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
impl<M, S> RunnableCommand for ServerArgs<M, S>
where
    M: MigratorTrait,
    S: ServerTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self.config.into_parsed_config(app_name).with_default(app_name).with_database(PhantomData::<M>).await;
        config.init_tracing_subscriber();
        let context = config.into_server_context(app_name, self.migrator).await.unwrap();
        S::serve(context).await.unwrap();
    }
}
