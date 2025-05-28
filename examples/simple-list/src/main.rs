use clap::Parser;

mod migration;
mod cli;
mod entity;

pub use lazy_supplements::error;
pub use lazy_supplements::global;

#[cfg(test)]
pub mod tests {
    use sea_orm::DatabaseConnection;
    use sea_orm_migration::MigratorTrait;

    use super::*;
    pub async fn get_or_init_temporary_database() -> &'static DatabaseConnection {
        global::GLOBAL.get_or_try_init_temporary_database(migration::Migrator).await.unwrap()
    }
}

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();
    println!("{:?}", args);
}
