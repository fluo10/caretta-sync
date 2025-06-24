use std::str::FromStr;

use chrono::{Days, Local};
use libp2p::{multiaddr, Multiaddr, PeerId};
use sea_orm::{entity::{
    prelude::*, *
}, sea_query};
use serde::{Deserialize, Serialize};

use crate::data::value::{MultiaddrValue, PeerIdValue};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "peer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(indexed)]
    pub created_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub updated_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub expires_at: DateTimeUtc,
    #[sea_orm(indexed)]
    pub peer_id: PeerIdValue,
    #[sea_orm(indexed)]
    pub address: MultiaddrValue,
}


#[derive(Copy, Clone, Debug, DeriveRelation, EnumIter)]
pub enum Relation {}
    
impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(peer_id: PeerId, multiaddr: Multiaddr) -> Self {
        let timestamp: DateTimeUtc = Local::now().to_utc();
        Self{
            peer_id: Set(PeerIdValue::from(peer_id)),
            address: Set(MultiaddrValue::from(multiaddr)),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            expires_at: Set(timestamp.checked_add_days(Days::new(30)).unwrap()),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::{cache::entity::peer, global::get_or_init_test_cache_database};

    use super::*;

    use libp2p::{identity::{self, Keypair}, swarm::handler::multi};

     #[tokio::test]
    async fn insert() {
        let db = get_or_init_test_cache_database().await;
        let peer_id = Keypair::generate_ed25519().public().to_peer_id();
        let multiaddr = Multiaddr::empty()
            .with(Ipv4Addr::new(127,0,0,1).into())
            .with(multiaddr::Protocol::Tcp(0));
        let inserted: Model = ActiveModel::new(peer_id.clone(), multiaddr.clone())
                .insert(db).await.unwrap();
        assert_eq!(PeerId::from(inserted.peer_id), peer_id);
        assert_eq!(Multiaddr::from(inserted.address), multiaddr);     
    }

}