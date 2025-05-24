#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(String),
}