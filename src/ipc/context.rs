use std::sync::Arc;

use iroh::Endpoint;
use iroh_docs::api::DocsApi;
use sea_orm::DatabaseConnection;

use crate::ipc::IpcActor;

#[derive(Debug)]
pub struct IpcContext {
    pub database_connection: Arc<DatabaseConnection>,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}

impl AsRef<DatabaseConnection> for IpcContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}

impl AsRef<Endpoint> for IpcContext {
    fn as_ref(&self) -> &Endpoint {
        &self.iroh_endpoint
    }
}

impl AsRef<DocsApi> for IpcContext {
    fn as_ref(&self) -> &DocsApi {
        &self.docs
    }
}