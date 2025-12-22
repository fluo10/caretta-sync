#[cfg(feature = "client")]
use rmcp::model::Implementation;

pub struct AppInfo {
    pub app_name: &'static str,
    #[cfg(feature = "client")]
    pub client_info: Implementation,
}
