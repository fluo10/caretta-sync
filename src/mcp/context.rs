use std::sync::Arc;

use iroh::Endpoint;
use iroh_docs::api::DocsApi;
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct McpContext {
    pub database_connection: Arc<DatabaseConnection>,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}

impl AsRef<DatabaseConnection> for McpContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}

impl AsRef<Endpoint> for McpContext {
    fn as_ref(&self) -> &Endpoint {
        &self.iroh_endpoint
    }
}

impl AsRef<DocsApi> for McpContext {
    fn as_ref(&self) -> &DocsApi {
        &self.docs
    }
}