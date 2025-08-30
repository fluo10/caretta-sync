use bevy::{app::{App, Plugin, Startup, Update}, ecs::{component::Component, query::With, system::{Commands, Query}}, tasks::TaskPool};
use caretta_sync_core::{cache::entity::{CachedPeerEntity, CachedPeerModel}, global::{CONFIG, DATABASE_CONNECTIONS}};
use caretta_sync_core::{
    proto::*,
};
use sea_orm::EntityTrait;

#[derive(Component)]
pub struct Peer;

#[derive(Component)]
pub struct PeerId(String);

#[derive(Component)]
pub struct PeerAddress(String);

#[tokio::main]
async fn add_cached_peers(mut commands: Commands) {
    let config = CONFIG.get_unchecked();
    let url = config.rpc.endpoint_url.to_string();
    let mut client = caretta_sync_core::proto::cached_peer_service_client::CachedPeerServiceClient::connect(url).await.expect("Unix socket should be accessible");
    let request = tonic::Request::new(CachedPeerListRequest {});
    let response = client.list(request).await.expect("Faild to request/response");
    let peers = response.into_inner().peers;
    for model in peers.into_iter() {
            commands.spawn((Peer, PeerId(model.peer_id.to_string())));
    }
}

fn print_peer(query: Query<&PeerId, With<Peer>>) {
    for peer_id in &query {
        println!("Hello {}!", peer_id.0);
    }
}

fn hello_world() {
    println!("hello world!");
}

pub struct PeerPlugin;

impl Plugin for PeerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_cached_peers);
        app.add_systems(Update, (hello_world, print_peer));
    }
}