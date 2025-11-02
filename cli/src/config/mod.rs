mod check;
mod list;

use check::*;
use list::*;

use caretta_sync_core::utils::runnable::Runnable;
use clap::{Args, Subcommand};
use sea_orm_migration::MigratorTrait;

#[derive(Debug, Args)]
pub struct ConfigCommandArgs<M>
where 
    M: MigratorTrait
{
    #[command(subcommand)]
    command: ConfigSubcommand<M>,
}

impl<M> Runnable for ConfigCommandArgs<M>
where 
    M: MigratorTrait
{
    fn run(self, app_name: &'static str) {
        self.command.run(app_name)
    }
}

#[derive(Debug, Subcommand)]
enum ConfigSubcommand<M>
where 
    M: MigratorTrait
{
    Check(ConfigCheckCommandArgs<M>),
    List(ConfigListCommandArgs<M>),
}

impl<M> Runnable for ConfigSubcommand<M>
where 
    M: MigratorTrait
{
    fn run(self, app_name: &'static str) {
        match self {
            Self::Check(x) => x.run(app_name),
            Self::List(x) => x.run(app_name),
        }
    }
}
