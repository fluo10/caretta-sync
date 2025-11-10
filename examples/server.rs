mod common;
use caretta_sync::{context::ServerContext, error::ServiceError, parser::ServerParser, server::ServerTrait, util::RunnableCommand};
use clap::Parser;
use common::APP_NAME;

#[derive(Debug)]
pub struct Migrator;

#[sea_orm_migration::async_trait::async_trait]
impl sea_orm_migration::MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![Box::new(
            caretta_sync::model::migration::m20220101_000001_create_table::Migration,
        )]
    }
}

#[derive(Debug)]
pub struct Server;

#[async_trait::async_trait]
impl ServerTrait for Server {
    async fn serve(context: ServerContext) -> Result<(), ServiceError> {
        caretta_sync::server::Server::new(context)
            .serve()
            .await
    }
}


fn main() {
    let args = ServerParser::<Migrator,Server>::parse();
    args.run(APP_NAME)
}