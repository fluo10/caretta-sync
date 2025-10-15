use std::sync::OnceLock;

use iroh::Endpoint;

pub static IROH_ENDPOINT: GlobalIrohEndpoint = GlobalIrohEndpoint::const_new();

pub struct GlobalIrohEndpoint {
    inner: OnceLock<Endpoint>,
}

impl GlobalIrohEndpoint {
    const fn const_new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }
    pub fn get_or_init(&self, endpoint: &Endpoint) -> Endpoint {
        self.inner.get_or_init(|| endpoint.clone()).clone()
    }
    pub fn get(&self) -> Option<Endpoint> {
        self.inner.get().map(|x| x.clone())
    }
    pub fn get_unchecked(&self) -> Endpoint {
        self.get()
            .expect("Global Iroh Endpoint must be initialized before use")
    }
}
