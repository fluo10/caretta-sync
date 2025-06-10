use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Node::up(manager).await?;
        RecordDeletion::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Node::down(manager).await?;
        RecordDeletion::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Node {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    SyncedAt,
    PeerId,
    Note,
}

#[async_trait::async_trait]
impl TableMigration for Node {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_uuid(Self::Id))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(timestamp_null(Self::SyncedAt))
                .col(string_len(Self::PeerId, 255))
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