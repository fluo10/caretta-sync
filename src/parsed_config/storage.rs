use std::path::PathBuf;

use clap::Args;
use serde::{Deserialize, Serialize};

use crate::util::{Emptiable, Mergeable};
/// A storage config parsed from file, args and enviroment variables
#[derive(Args, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParsedStorageConfig {
    #[arg(long, env)]
    pub data_dir: Option<PathBuf>,
    #[arg(long, env)]
    pub cache_dir: Option<PathBuf>,
}

impl ParsedStorageConfig {
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

impl Emptiable for ParsedStorageConfig {
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
impl Mergeable for ParsedStorageConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.data_dir.take() {
            let _ = self.data_dir.insert(x);
        };
        if let Some(x) = other.cache_dir.take() {
            let _ = self.cache_dir.insert(x);
        };
    }
}

#[cfg(feature = "server")]
mod server {
    use super::*;
    use crate::config::StorageConfig;
    use crate::parsed_config::error::ParsedConfigError;
    impl TryFrom<ParsedStorageConfig> for StorageConfig {
        type Error = ParsedConfigError;

        fn try_from(value: ParsedStorageConfig) -> Result<Self, Self::Error> {
            Ok(Self {
                data_dir: value
                    .data_dir
                    .ok_or(ParsedConfigError::MissingConfig("data_dir"))?,
                cache_dir: value
                    .cache_dir
                    .ok_or(ParsedConfigError::MissingConfig("cache_dir"))?,
            })
        }
    }
    impl From<StorageConfig> for ParsedStorageConfig {
        fn from(config: StorageConfig) -> ParsedStorageConfig {
            Self {
                data_dir: Some(config.data_dir),
                cache_dir: Some(config.cache_dir),
            }
        }
    }
}
