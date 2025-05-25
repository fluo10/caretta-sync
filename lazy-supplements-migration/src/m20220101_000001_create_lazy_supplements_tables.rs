use sea_orm_migration::{prelude::*, schema::*};

use crate::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Device::up(manager).await?;
        RecordDeletion::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Device::down(manager).await?;
        RecordDeletion::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Device {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    SyncedAt,
    Name,
    Note,
}

#[async_trait::async_trait]
impl TableMigration for Device {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_uuid(Self::Id))
                .col(timestamp_with_time_zone(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(timestamp_with_time_zone_null(Self::SyncedAt))
                .col(string(Self::Name))
                .col(text(Self::Note))
                .to_owned()
        ).await?;
        Ok(())


    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}

#[derive(DeriveIden, DeriveMigrationName)]
enum RecordDeletion {
    Table,
    Id,
    CreatedAt,
    TableName, 
    RecordId,
}

#[async_trait::async_trait]
impl TableMigration for RecordDeletion {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_uuid(Self::Id))
                .col(timestamp_with_time_zone(Self::CreatedAt))
                .col(string(Self::TableName))
                .col(uuid(Self::RecordId))
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}