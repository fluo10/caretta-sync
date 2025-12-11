pub trait AsIrohEndpoint {
    fn as_iroh_endpoint(&self) -> &iroh::Endpoint;
}

impl AsIrohEndpoint for  iroh::Endpoint {
    fn as_iroh_endpoint(&self) -> &iroh::Endpoint {
        self
    }
}