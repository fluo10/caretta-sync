use std::sync::Arc;

use iroh::Endpoint;
use iroh_docs::api::DocsApi;
use sea_orm::DatabaseConnection;

use crate::types::{AppDatabase, Database};

#[derive(Clone, Debug)]
pub struct ServiceGenerator {
    pub app_database: Arc<AppDatabase>,
    pub database: Arc<Database>,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}
