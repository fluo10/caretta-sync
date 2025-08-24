use caretta_sync::{bevy::peer::PeerPlugin, utils::Runnable};
use bevy::prelude::*;

pub struct Gui {}

impl Runnable for Gui {
    fn run(self, app_name: &'static str) {
        App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugins(PeerPlugin)
        .run();
    }
}