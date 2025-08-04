mod trusted_node;
mod record_deletion;

pub use trusted_node::{
    ActiveModel as TrustedNodeActiveModel,
    Column as TrustedNodeColumn,
    Entity as TrustedNodeEntity,
    Model as TrustedNodeModel,
};

pub use record_deletion::{
    ActiveModel as RecordDeletionActiveModel,
    Column as RecordDeletionColumn,
    Entity as RecordDeletionEntity,
    Model as RecordDeletionModel,
};

#[cfg(test)]
mod tests {
    use crate::{data::value::PeerIdValue, global::{generate_uuid, get_or_init_test_data_database}};

    use super::*;

    use libp2p::{identity, PeerId};
    use sea_orm::ActiveModelTrait;

     #[tokio::test]
    async fn check_insert() {
        let db = get_or_init_test_data_database().await;
        
        let node = TrustedNodeActiveModel::new(PeerId::random(), "test note".to_owned()).insert(db).await.unwrap();
        let _ = RecordDeletionActiveModel::new(node.id, "test_table".to_string(), generate_uuid()).insert(db).await.unwrap();
    }

}