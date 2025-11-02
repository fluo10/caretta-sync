use std::marker::PhantomData;

use caretta_sync_core::{ server::{Server, ServerTrait}};
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
        let context = self.config.into_server_context(app_name,PhantomData::<M>).await;
        context.init_tracing_subscriber();
        S::serve(context).await.unwrap();
    }
}
