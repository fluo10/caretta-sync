use sea_orm::{prelude::*, query::*, sea_query::SimpleExpr, *};
#[cfg(feature="macros")]
pub use caretta_macros::SyncableModel;
pub trait SyncableModel: ModelTrait<Entity = Self::SyncableEntity> {
    type SyncableEntity: SyncableEntity<SyncableModel = Self>;
    fn get_timestamp(&self) -> DateTimeUtc;
    fn get_id(&self) -> Uuid;
    fn get_author_id(&self) -> Uuid;
}

pub trait SyncableEntity: EntityTrait<
    Model = Self::SyncableModel,
    ActiveModel = Self::SyncableActiveModel,
    Column = Self::SyncableColumn,
>{
    type SyncableModel: SyncableModel<SyncableEntity = Self> + FromQueryResult;
    type SyncableActiveModel: SyncableActiveModel<SyncableEntity= Self>;
    type SyncableColumn: SyncableColumn;

    async fn get_updated(from: DateTimeUtc, db: &DatabaseConnection) -> Result<Vec<<Self as EntityTrait>::Model>, SyncableError> {
        let result: Vec<Self::SyncableModel> = <Self as EntityTrait>::find()
            .filter(Self::SyncableColumn::timestamp_after(from))
            .all(db)
            .await.unwrap();
        Ok(result)
    }
    async fn get_updated_by(author: Uuid, from: DateTimeUtc, db: &DatabaseConnection) -> Result<Vec<<Self as EntityTrait>::Model>, SyncableError> {
        let result: Vec<Self::SyncableModel> = <Self as EntityTrait>::find()
            .filter(Self::SyncableColumn::timestamp_after(from))
            .filter(Self::SyncableColumn::author_id_eq(author))
            .all(db)
            .await.unwrap();
        Ok(result)
    }
    fn apply_updated(models: Vec<<Self as EntityTrait>::Model>, db: &DatabaseConnection) {
        todo!()
    }
}

pub trait SyncableActiveModel: ActiveModelTrait<Entity = Self::SyncableEntity> {
    
    type SyncableEntity: SyncableEntity<SyncableActiveModel = Self>;
    fn get_id(&self) -> Option<Uuid>;
    fn get_timestamp(&self) -> Option<DateTimeUtc>;
    fn get_author_id(&self) -> Option<Uuid>;
    fn try_merge(&mut self, other: <Self::SyncableEntity as SyncableEntity>::SyncableModel) -> Result<(), SyncableError> {
        if self.get_id().ok_or(SyncableError::MissingField("uuid"))? != other.get_id() {
            return Err(SyncableError::MismatchUuid)
        }
        if self.get_timestamp().ok_or(SyncableError::MissingField("updated_at"))? < other.get_timestamp() {
            for column in <<<Self as ActiveModelTrait>::Entity as EntityTrait>::Column as Iterable>::iter() {
                if column.should_synced(){
                    self.take(column).set_if_not_equals(other.get(column));
                }
            }
        }
        Ok(())
    }

}

pub trait SyncableColumn: ColumnTrait {
    fn is_id(&self) -> bool;
    fn is_timestamp(&self) -> bool;
    fn should_synced(&self) -> bool;
    fn timestamp_after(from: DateTimeUtc) -> SimpleExpr;
    fn author_id_eq(author_id: Uuid) -> SimpleExpr;
    fn is_author_id(&self) -> bool;
}


#[derive(Debug, thiserror::Error)]
pub enum SyncableError {
    #[error("Invalid UUID")]
    MismatchUuid,
    #[error("mandatory field {0} is missing")]
    MissingField(&'static str),
    
}