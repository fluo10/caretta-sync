//! Structs about cached remote_node.

use sea_orm::entity::prelude::*;
use tripod_id::Double;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};

use uuid::Uuid;

use crate::data::local::types::PublicKeyBlob;

/// RemoteNode information cached in local database.
/// 
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - Acutualy this is some sort of junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual remote_node information is managed by iroh endpoint and not contained in this model.
/// - Once a remote_node is authorized, it is assigned a global (=synced) ID as authorized_remote_node so essentially this local id targets unauthorized remote_nodes.
///
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "remote_node")]
pub struct Model {

    /// serial primary key.
    #[sea_orm(primary_key)]
    pub id: u32,

    /// public tripod id of remote_node.
    /// this id is use only the node itself and not synced so another node has different local_remote_node_id even if its public_key is same.
    pub public_id: Double,

    /// Iroh public key
    pub public_key: PublicKeyBlob,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}