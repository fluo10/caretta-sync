mod multi_address;
mod peer;

pub use multi_address::{
    ActiveModel as ActiveMultiAddressModel,
    Column as MultiAddressColumn,
    Model as MultiAddressModel,
    Entity as MultiAddressEntity,
};

pub use peer::{
    ActiveModel as ActivePeerModel,
    Column as PeerColumn,
    Model as PeerModel,
    Entity as PeerEntity,
};