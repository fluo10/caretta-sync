use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Peer::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Peer::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden, DeriveMigrationName)]
enum Peer {
    Table,
    Id,
    PeerId,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
    Address,
}

static IDX_PEER_ADDRESS: &str = "idx_peer_address";
static IDX_PEER_PEER_ID: &str = "idx_peer_peer_id";
static IDX_PEER_CREATED_AT: &str = "idx_peer_created_at";
static IDX_PEER_UPDATED_AT: &str = "idx_peer_updated_at";
static IDX_PEER_EXPIRES_AT: &str = "idx_peer_expires_at";

#[async_trait::async_trait]
impl TableMigration for Peer {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_auto(Self::Id))
                .col(string_len(Self::PeerId, 255))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(timestamp(Self::ExpiresAt))
                .col(text_uniq(Self::Address))
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_PEER_ID)
                .table(Self::Table)
                .col(Self::PeerId)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_ADDRESS)
                .table(Self::Table)
                .col(Self::Address)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_EXPIRES_AT)
                .table(Self::Table)
                .col(Self::ExpiresAt)
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}