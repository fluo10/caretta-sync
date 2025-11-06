use std::marker::PhantomData;

use caretta_sync_core::{ context::ServerContext, server::{Server, ServerTrait}};
use clap::Args;
use sea_orm_migration::MigratorTrait;

use crate::{RunnableCommand, option::ConfigOptionArgs};

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
    config: ConfigOptionArgs,
}
impl<M, S> RunnableCommand for ServeCommandArgs<M, S>
where
    M: MigratorTrait,
    S: ServerTrait,
{
    #[tokio::main]
    async fn run(self, app_name: &'static str) {
        let config = self.config.into_parsed_config(app_name).with_default(app_name).with_database(PhantomData::<M>).await.unwrap();
        config.init_tracing_subscriber();
        let context = ServerContext::new(app_name, config, PhantomData::<M>).await.unwrap();
        S::serve(context).await.unwrap();
    }
}
