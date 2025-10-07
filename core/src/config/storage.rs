use std::path::PathBuf;

#[cfg(feature="cli")]
use clap::Args;

#[cfg(any(test, feature="test"))]
use tempfile::tempdir;
use crate::{config::{ConfigError, PartialConfig}, utils::{emptiable::Emptiable, get_binary_name, mergeable::Mergeable}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct StorageConfig {
    pub data_directory: PathBuf,
    pub cache_directory: PathBuf,
}

impl StorageConfig {
    pub fn get_global_data_directory(&self) -> PathBuf {
        self.data_directory.join("global")
    }
    pub fn get_global_root_document_path(&self) -> PathBuf {
        self.data_directory.join("global.bin")
    }
    pub fn get_local_data_directory(&self) -> PathBuf {
        self.data_directory.join("local")
    }
    pub fn get_local_database_path(&self) -> PathBuf {
        self.data_directory.join("local.sqlite")
    }
}

impl TryFrom<PartialStorageConfig> for StorageConfig {
    type Error = ConfigError;

    fn try_from(value: PartialStorageConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            data_directory: value.data_directory.ok_or(ConfigError::MissingConfig("data_directory".to_string()))?,
            cache_directory: value.cache_directory.ok_or(ConfigError::MissingConfig("cache_directory".to_string()))?,
        })
    }
}

#[cfg_attr(feature="cli", derive(Args))]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialStorageConfig {
    #[cfg_attr(feature="cli", arg(long))]
    pub data_directory: Option<PathBuf>,
    #[cfg_attr(feature="cli", arg(long))]
    pub cache_directory: Option<PathBuf>,
}

impl PartialStorageConfig {
    #[cfg(not(any(target_os="android", target_os="ios")))]
    pub fn default(app_name: &'static str) -> Self {
    
        let mut data_dir = dirs::data_local_dir().unwrap();
        data_dir.push(app_name);
        let mut cache_dir = dirs::cache_dir().unwrap();
        cache_dir.push(app_name);

        Self {
            data_directory: Some(data_dir),
            cache_directory: Some(cache_dir)
        }
    }
    #[cfg(target_os="android")]
    pub fn default_android() -> Self{
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
    #[cfg(target_os="ios")]
    pub fn default(_: &'static str) -> Self{
        
        use objc2::rc::Retained;
        use objc2::msg_send;
        use objc2_foundation::*;

        let home_dir: Retained<NSString> = unsafe {NSHomeDirectory()};
        
        let path = PathBuf::from(home_dir.to_string());
        Self {
            data_directory: Some(path.join("Library")),
            cache_directory: Some(path.join("Library").join("Cache")),
        }

    
    }

}

impl From<StorageConfig> for PartialStorageConfig {
    fn from(config: StorageConfig) -> PartialStorageConfig {
        Self {
            data_directory: Some(config.data_directory),
            cache_directory: Some(config.cache_directory),
        }
    }
}

impl Emptiable for PartialStorageConfig {
    fn empty() -> Self {
        Self {
            data_directory: None,
            cache_directory: None
        }
    }

    fn is_empty(&self) -> bool {
        self.data_directory.is_none() && self.cache_directory.is_none()
    }
}
impl Mergeable for PartialStorageConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.data_directory.take() {
            let _ = self.data_directory.insert(x);
        };
        if let Some(x) = other.cache_directory.take() {
            let _ = self.cache_directory.insert(x);
        };
    }
}

impl Mergeable for Option<PartialStorageConfig> {
    fn merge(&mut self, mut other: Self) {
        match other.take() {
            Some(x) => {
                if let Some(y) = self.as_mut() {
                    y.merge(x);
                } else {
                    let _ = self.insert(x);
                }
            },
            None => {}
        };
    }
}