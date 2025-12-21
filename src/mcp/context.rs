use std::sync::{Arc, OnceLock};

use iroh::Endpoint;
use iroh_docs::api::DocsApi;
use sea_orm::DatabaseConnection;

use crate::{types::{AppDatabase, Database}};

#[derive(Debug)]
pub struct Context {
    pub app_database: AppDatabase,
    pub database: Database,
    pub iroh_endpoint: Endpoint,
    pub docs: DocsApi,
}
