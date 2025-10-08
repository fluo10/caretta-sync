use std::os::unix::raw::time_t;

use mtid::Dtid;
use chrono::{DateTime, Local, NaiveDateTime};
use iroh::{NodeId, PublicKey};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Request of node authorization.
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="authorization_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    uuid: Uuid,
    public_id: Dtid,
    remote_node_id: u32,
    created_at: DateTime<Local>,
    closed_at: Option<DateTime<Local>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RemoteNode,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RemoteNode => Entity::belongs_to(super::remote_node::Entity)
                .from(Column::RemoteNodeId)
                .to(super::remote_node::Column::Id)
                .into()
        }
    }
}

impl Related<super::remote_node::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RemoteNode.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

