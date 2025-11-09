use url::Url;

#[cfg(unix)]
static DEFAULT_PORT: u16 = 54321;

#[derive(Clone, Debug)]
pub struct RpcConfig {
    pub endpoint_url: Url,
}
