#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB Error: {0}")]
    Db(#[from]sea_orm::DbErr),
    #[error("mandatory config `{0}` is missing")]
    MissingConfig(String),
}