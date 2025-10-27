use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    tasks::TaskPool,
};
use caretta_sync_core::proto::*;

#[derive(Component)]
pub struct Peer;

#[derive(Component)]
pub struct PeerId(String);

#[derive(Component)]
pub struct PeerAddress(String);

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
        // app.add_systems(Startup, add_cached_peers);
        app.add_systems(Update, (hello_world, print_peer));
    }
}
