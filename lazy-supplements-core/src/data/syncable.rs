use sea_orm::{*, prelude::*, query::*};
pub use lazy_supplements_macros::SyncableModel;
pub trait SyncableModel: ModelTrait<Entity = Self::SyncableEntity> {
    type SyncableEntity: SyncableEntity<SyncableModel = Self>;
    fn get_timestamp(&self) -> DateTimeUtc;
    fn get_uuid(&self) -> Uuid;
}

pub trait SyncableEntity: EntityTrait<
    Model = Self::SyncableModel,
    ActiveModel = Self::SyncableActiveModel,
    Column = Self::SyncableColumn,
>{
    type SyncableModel: SyncableModel<SyncableEntity = Self> + FromQueryResult;
    type SyncableActiveModel: SyncableActiveModel<SyncableEntity= Self>;
    type SyncableColumn: SyncableColumn;

    async fn get_updated_after(date: DateTimeUtc, db: &DatabaseConnection) -> Result<Vec<<Self as EntityTrait>::Model>, SyncableError> {
        let result: Vec<Self::SyncableModel> = <Self as EntityTrait>::find()
            .filter(Self::SyncableColumn::updated_at().gte(date))
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
    fn get_uuid(&self) -> Option<Uuid>;
    fn get_timestamp(&self) -> Option<DateTimeUtc>;
    fn try_merge(&mut self, other: <Self::SyncableEntity as SyncableEntity>::SyncableModel) -> Result<(), SyncableError> {
        if self.get_uuid().ok_or(SyncableError::MissingField("uuid"))? != other.get_uuid() {
            return Err(SyncableError::MismatchUuid)
        }
        if self.get_timestamp().ok_or(SyncableError::MissingField("updated_at"))? < other.get_timestamp() {
            for column in <<<Self as ActiveModelTrait>::Entity as EntityTrait>::Column as Iterable>::iter() {
                self.take(column).set_if_not_equals(other.get(column));
            }
        }
        Ok(())
    }

}

pub trait SyncableColumn: ColumnTrait {
    fn is_uuid(&self) -> bool;
    fn is_timestamp(&self) -> bool;
    fn updated_at() -> Self;
    fn should_skipped(&self);
}


#[derive(Debug, thiserror::Error)]
pub enum SyncableError {
    #[error("Invalid UUID")]
    MismatchUuid,
    #[error("mandatory field {0} is missing")]
    MissingField(&'static str),
    
}