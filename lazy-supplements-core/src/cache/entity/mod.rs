mod multi_address;
mod peer;

pub use multi_address::{
    ActiveModel as MultiAddressActiveModel,
    Model as MultiAddressModel,
    Entity as MultiAddressEntity,
};

pub use peer::{
    ActiveModel as PeerActiveModel,
    Model as PeerModel,
    Entity as PeerEntity,
};