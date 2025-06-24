mod trusted_peer;
mod record_deletion;

pub use trusted_peer::{
    ActiveModel as TrustedPeerActiveModel,
    Column as TrustedPeerColumn,
    Entity as TrustedPeerEntity,
    Model as TrustedPeerModel,
};

pub use record_deletion::{
    ActiveModel as RecordDeletionActiveModel,
    Column as RecordDeletionColumn,
    Entity as RecordDeletionEntity,
    Model as RecordDeletionModel,
};
