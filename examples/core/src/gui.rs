use bevy::prelude::*;
use caretta_sync::{bevy::peer::PeerPlugin, cli::RunnableCommand};

pub struct Gui {}

impl RunnableCommand for Gui {
    fn run(self, app_name: &'static str) {
        App::new()
            //.add_plugins(DefaultPlugins)
            .add_plugins(PeerPlugin)
            .run();
    }
}
