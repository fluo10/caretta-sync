mod node;
mod record_deletion;

pub use node::{
    ActiveModel as NodeActiveModel,
    Column as NodeColumn,
    Entity as NodeEntity,
    Model as NodeModel,
};

pub use record_deletion::{
    ActiveModel as RecordDeletionActiveModel,
    Column as RecordDeletionColumn,
    Entity as RecordDeletionEntity,
    Model as RecordDeletionModel,
};
use uuid::{ContextV7, Timestamp, Uuid};

pub fn generate_uuid() -> Uuid {
    Uuid::new_v7(Timestamp::now(ContextV7::new()))
}