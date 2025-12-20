use crate::config::McpConfig;

/// A Config for desktop client
/// 
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub mcp: McpConfig
}