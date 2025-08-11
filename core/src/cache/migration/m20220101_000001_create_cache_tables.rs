use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        CachedPeer::up(manager).await?;
        CachedAddress::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        CachedAddress::down(manager).await?;
        CachedPeer::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden, DeriveMigrationName)]
enum CachedPeer {
    Table,
    Id,
    PeerId,
    CreatedAt,
    UpdatedAt,
}

static IDX_CACHED_PEER_PEER_ID: &str = "idx_cached_peer_peer_id";
static IDX_CACHED_PEER_CREATED_AT: &str = "idx_cached_peer_created_at";
static IDX_CACHED_PEER_UPDATED_AT: &str = "idx_cached_peer_updated_at";

#[async_trait::async_trait]
impl TableMigration for CachedPeer {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_auto(Self::Id))
                .col(string_len(Self::PeerId, 255))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_PEER_PEER_ID)
                .table(Self::Table)
                .col(Self::PeerId)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_PEER_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_PEER_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}

#[derive(DeriveIden, DeriveMigrationName)]
enum CachedAddress {
    Table,
    Id,
    CachedPeerId,
    CreatedAt,
    UpdatedAt,
    Multiaddress,
}

static IDX_CACHED_ADDRESS_MULTIADDRESS: &str = "idx_cached_address_multiaddress";
static IDX_CACHED_ADDRESS_CACHED_PEER_ID: &str = "idx_cached_address_cached_peer_id";
static IDX_CACHED_ADDRESS_CREATED_AT: &str = "idx_cached_address_created_at";
static IDX_CACHED_ADDRESS_UPDATED_AT: &str = "idx_cached_address_updated_at";
static FK_CACHED_ADDRESS_CACHED_PEER: &str = "fk_cached_address_cached_peer";

#[async_trait::async_trait]
impl TableMigration for CachedAddress {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_auto(Self::Id))
                .col(integer(Self::CachedPeerId))
                .foreign_key(ForeignKey::create()
                    .name(FK_CACHED_ADDRESS_CACHED_PEER)
                    .from(Self::Table,Self::CachedPeerId)
                    .to(CachedPeer::Table, CachedPeer::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                )
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(text_uniq(Self::Multiaddress))
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_ADDRESS_CACHED_PEER_ID)
                .table(Self::Table)
                .col(Self::CachedPeerId)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_ADDRESS_MULTIADDRESS)
                .table(Self::Table)
                .col(Self::Multiaddress)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_ADDRESS_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_CACHED_ADDRESS_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned()
        ).await?;

        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}