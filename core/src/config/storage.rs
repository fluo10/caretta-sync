use std::{marker::PhantomData, path::PathBuf};

#[cfg(feature = "cli")]
use clap::Args;
//use iroh_docs::store::Store;
use sea_orm::{sqlx::database, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::{
    config::ConfigError, error::Error, utils::{emptiable::Emptiable, mergeable::Mergeable}
};
use serde::{Deserialize, Serialize};
#[cfg(any(test, feature = "test"))]
use tempfile::tempdir;

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl StorageConfig {
    const DATABASE_FILE_NAME: &str = "database.sqlite";
    const DOCS_FILE_NAME: &str = "docs.bin";

    pub fn to_docs_path(&self) -> PathBuf {
        self.data_dir.join(Self::DOCS_FILE_NAME)
    }
    // pub fn to_docs(&self) -> Result<Store, Error> {
    //     Ok(Store::persistent(self.to_docs_path()).map_err(|e| Error::DocsOpen(e))?)
    // }
    pub fn to_database_path(&self) -> PathBuf {
        self.data_dir.join(Self::DATABASE_FILE_NAME)
    }
    pub async fn to_database_connection<T>(&self, _: PhantomData<T>) -> Result<DatabaseConnection,ConfigError>
    where T: MigratorTrait {
        let database_path = self.to_database_path();
        if let Some(x) = database_path.parent() {
            std::fs::create_dir_all(x)?;
        }
        let url = "sqlite://".to_owned()
            + self.to_database_path().to_str().expect("Invalid path string")
            + "?mode=rwc";
        let db = Database::connect(url).await?;
        T::up(&db, None).await?;
        Ok(db)
    }
}

impl TryFrom<PartialStorageConfig> for StorageConfig {
    type Error = ConfigError;

    fn try_from(value: PartialStorageConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            data_dir: value
                .data_dir
                .ok_or(ConfigError::MissingConfig("data_dir"))?,
            cache_dir: value
                .cache_dir
                .ok_or(ConfigError::MissingConfig("cache_dir"))?,
        })
    }
}

#[cfg_attr(feature = "cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialStorageConfig {
    #[cfg_attr(feature = "cli", arg(long))]
    pub data_dir: Option<PathBuf>,
    #[cfg_attr(feature = "cli", arg(long))]
    pub cache_dir: Option<PathBuf>,
}

impl PartialStorageConfig {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    pub fn default(app_name: &'static str) -> Self {
        let mut data_dir = dirs::data_local_dir().unwrap();
        data_dir.push(app_name);
        let mut cache_dir = dirs::cache_dir().unwrap();
        cache_dir.push(app_name);

        Self {
            data_dir: Some(data_dir),
            cache_dir: Some(cache_dir),
        }
    }
    #[cfg(target_os = "android")]
    pub fn default_android() -> Self {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let ctx = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let cache_dir = env
            .call_method(ctx, "getFilesDir", "()Ljava/io/File;", &[])?
            .l()?;
        let cache_dir: jni::objects::JString = env
            .call_method(&cache_dir, "toString", "()Ljava/lang/String;", &[])?
            .l()?
            .try_into()?;
        let cache_dir = env.get_string(&cache_dir)?;
        let cache_dir = cache_dir.to_str()?;
        Ok(cache_dir.to_string())
    }
    #[cfg(target_os = "ios")]
    pub fn default(_: &'static str) -> Self {
        use objc2::msg_send;
        use objc2::rc::Retained;
        use objc2_foundation::*;

        let home_dir: Retained<NSString> = unsafe { NSHomeDirectory() };

        let path = PathBuf::from(home_dir.to_string());
        Self {
            data_dir: Some(path.join("Library")),
            cache_dir: Some(path.join("Library").join("Cache")),
        }
    }
}

impl From<StorageConfig> for PartialStorageConfig {
    fn from(config: StorageConfig) -> PartialStorageConfig {
        Self {
            data_dir: Some(config.data_dir),
            cache_dir: Some(config.cache_dir),
        }
    }
}

impl Emptiable for PartialStorageConfig {
    fn empty() -> Self {
        Self {
            data_dir: None,
            cache_dir: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.data_dir.is_none() && self.cache_dir.is_none()
    }
}
impl Mergeable for PartialStorageConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.data_dir.take() {
            let _ = self.data_dir.insert(x);
        };
        if let Some(x) = other.cache_dir.take() {
            let _ = self.cache_dir.insert(x);
        };
    }
}

impl Mergeable for Option<PartialStorageConfig> {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.take() {
            if let Some(y) = self.as_mut() {
                y.merge(x);
            } else {
                let _ = self.insert(x);
            }
        };
    }
}
