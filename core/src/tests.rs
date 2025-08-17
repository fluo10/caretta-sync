use std::{path::PathBuf, sync::LazyLock};

use sea_orm::{sea_query::{FromValueTuple, IntoValueTuple, ValueType}, ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, PrimaryKeyToColumn, PrimaryKeyTrait, Value};
use sea_orm::QueryFilter;
use tempfile::TempDir;
use crate::{ config::PartialConfig, message::Message};

use serde::{de::DeserializeOwned, Deserialize, Serialize};


pub static TEST_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let pkg_name = env!("CARGO_PKG_NAME");
    let timestamp = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, false);
    std::env::temp_dir().join(pkg_name).join( &timestamp)
});

pub static TEST_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    TempDir::new().unwrap().keep()
});

pub static TEST_DATABASE_PATH: std::sync::LazyLock<PathBuf> = std::sync::LazyLock::new(|| {
    TEST_DIR_PATH.join("lazy-supplements.sqlite")
});

pub trait TestDefault {
    fn test_default() -> Self;
}

pub trait GlobalTestDefault<T: 'static> {
    async fn get_or_init_test_default(&'static self) -> &'static T; 
}
