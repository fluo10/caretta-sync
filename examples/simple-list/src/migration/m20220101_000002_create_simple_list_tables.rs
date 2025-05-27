use sea_orm_migration::{prelude::*, schema::*};

use lazy_supplements_migration::TableMigration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ListItem::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ListItem::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum ListItem {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    IsTrashed,
    Content,
}

#[async_trait::async_trait]
impl TableMigration for ListItem {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Self::Table)
                .if_not_exists()
                .col(pk_uuid(Self::Id))
                .col(timestamp(Self::CreatedAt))
                .col(timestamp(Self::UpdatedAt))
                .col(boolean(Self::IsTrashed))
                .col(string_len(Self::Content, 255))
                .to_owned()
        ).await?;
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr>{
        manager.drop_table(Table::drop().table(Self::Table).to_owned()).await
    }
}
