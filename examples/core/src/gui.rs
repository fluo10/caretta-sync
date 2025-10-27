use bevy::prelude::*;
use caretta_sync::{bevy::peer::PeerPlugin, utils::Runnable};

pub struct Gui {}

impl Runnable for Gui {
    fn run(self, app_name: &'static str) {
        App::new()
            //.add_plugins(DefaultPlugins)
            .add_plugins(PeerPlugin)
            .run();
    }
}
