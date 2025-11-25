//! Structs about cached remote_node.

use iroh::PublicKey;
use caretta_id::CarettaId;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::model::types::PublicKeyBlob;

/// RemoteNode information cached in local database.
///
/// - Currently this only contain local uid and public key (=node id) of iroh.
/// - Acutualy this is some sort of junction table enable to use caretta-id to specify items in the UI, especially on the CLI.
/// - Actual remote_node information is managed by iroh endpoint and not contained in this model.
/// - Once a remote_node is authorized, it is assigned a global (=synced) ID as authorized_remote_node so essentially this local id targets unauthorized remote_nodes.
///
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "authorized_device")]
pub struct AuthorizedDevice {

    pub id: Uuid,

    /// public [`CarettaId`] of the node.
    pub public_id: CarettaId,

    /// Iroh public key
    pub public_key: PublicKey,

    /// Name of the node.
    pub name: String,

    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}