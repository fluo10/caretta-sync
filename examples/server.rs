mod common;
use caretta_sync::{context::ServerContext, error::ServiceError, parser::ServerParser, server::ServerTrait, util::RunnableCommand};
use clap::Parser;
use common::APP_NAME;

#[derive(Debug)]
pub struct Server;

#[async_trait::async_trait]
impl ServerTrait for Server {
    async fn serve(context: ServerContext) -> Result<(), ServiceError> {
        Ok(caretta_sync::server::Server::new(context)
            .serve()
            .await)
    }
}


fn main() {
    let args = ServerParser::<Server>::parse();
    args.run(APP_NAME)
}