use sea_orm_migration::{prelude::*, schema::*};

use crate::migration::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Peer::up(manager).await?;
        Address::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Peer::down(manager).await?;
        Address::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Peer {
    Table,
    Id,
    PeerId,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
}

static IDX_PEER_PEER_ID: &str = "idx_peer_peer_id";

#[async_trait::async_trait]
impl TableMigration for Peer {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_auto(Self::Id))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(timestamp(Self::ExpiresAt))
                .col(string_len_uniq(Self::PeerId, 255))
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_PEER_PEER_ID)
                .table(Self::Table)
                .col(Self::PeerId)
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}

#[derive(DeriveIden, DeriveMigrationName)]
enum Address {
    Table,
    Id,
    PeerId,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
    MultiAddress,
}

static IDX_ADDRESS_MULTIADDRESS: &str = "idx_address_multiaddress";
static FK_ADDRESS_PEER: &str = "fk_address_peer";

#[async_trait::async_trait]
impl TableMigration for Address {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_auto(Self::Id))
                .col(integer(Self::PeerId))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(timestamp(Self::ExpiresAt))
                .col(text_uniq(Self::MultiAddress))
                .foreign_key(
                    ForeignKey::create()
                        .name(FK_ADDRESS_PEER)
                        .from(Self::Table, Self::PeerId)
                        .to(Peer::Table, Peer::Id)
                )
                .to_owned()
        ).await?;
        manager.create_index(
            Index::create()
                .name(IDX_ADDRESS_MULTIADDRESS)
                .table(Self::Table)
                .col(Self::MultiAddress)
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}