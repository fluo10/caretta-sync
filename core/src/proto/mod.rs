pub mod authorization_request;
mod authorized_node;
mod remote_node;
mod common;
mod error;

//mod server;
mod generated{
    tonic::include_proto!("caretta_sync");
}

pub use generated::{
    *,
    PublicKey as PublicKeyMessage,
    Uuid as UuidMessage,
    Url as UrlMessage,
    SocketAddr as SocketAddrMessage,
    SocketAddrV6 as SocketAddrV6Message,
    SocketAddrV4 as SocketAddrV4Message,
    Ipv4Addr as Ipv4AddrMessage,
    Ipv6Addr as Ipv6AddrMessage,
};

pub use common::*;
pub use error::*;
pub use remote_node::*;