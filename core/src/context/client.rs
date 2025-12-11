use crate::{
    config::{IpcConfig},
    error::CoreError,
};

/// A context for client
#[derive(Clone, Debug)]
pub struct ClientContext {
    pub app_name: &'static str,
    pub ipc_config: IpcConfig,
}

impl AsRef<ClientContext> for ClientContext {
    fn as_ref(&self) -> &ClientContext {
        self
    }
}
