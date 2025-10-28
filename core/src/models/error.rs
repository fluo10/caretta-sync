use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum ModelsError{
    #[error("Db error: {0}")]
    Db(#[from] DbErr),
}