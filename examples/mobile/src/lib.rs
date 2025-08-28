use bevy::{
    color::palettes::basic::*,
    input::{gestures::RotationGesture, touch::TouchPhase},
    log::{Level, LogPlugin},
    prelude::*,
    window::{AppLifecycle, ScreenEdge, WindowMode},
    winit::WinitSettings,
};
use caretta_sync::{config::{Config, PartialConfig, PartialP2pConfig, PartialStorageConfig, StorageConfig}, server::ServerTrait, utils::{Emptiable, Mergeable}};
use caretta_sync_example_core::{global::APP_NAME, server::Server};

#[tokio::main]
pub async fn init_config() {
    let storage_config: StorageConfig = PartialStorageConfig::default(APP_NAME).try_into().unwrap();
    let config_path = storage_config.data_directory.join("config.toml");
    let mut config = PartialConfig::read_from(&config_path).await.unwrap();
    if let Some(x) = if let Some(y) = config.p2p.as_mut() {
        if y.private_key.is_none() {
            Some(y.clone().with_new_private_key())
        } else {
            None
        }
        
    } else {
        Some(PartialP2pConfig::empty().with_new_private_key())
    } {
        config.p2p = Some(x);
        config.write_to(&config_path).await.unwrap()
    }
    let mut default = PartialConfig::default(APP_NAME);
    default.merge(config);
    let config2 : Config = default.try_into().unwrap();
    Server::serve_all(&config2).await;
    
}

#[bevy_main]
pub fn main() {

    //init_config();


    
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs_trace".to_string(),
            ..Default::default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                recognize_rotation_gesture: true,
                prefers_home_indicator_hidden: true,
                prefers_status_bar_hidden: true,
                preferred_screen_edges_deferring_system_gestures: ScreenEdge::Bottom,
                ..default()
            }),
            ..default()
        }),
    )
    .insert_resource(WinitSettings::mobile())
    .add_systems(Startup, setup_scene)
    .run();
}

fn setup_scene(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // MSAA makes some Android devices panic, this is under investigation
        // https://github.com/bevyengine/bevy/issues/8229
        #[cfg(target_os = "android")]
        Msaa::Off,
    ));
    commands
        .spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            }
        ))
        .with_child((
            Text::new(format!("{:?}", PartialConfig::default(APP_NAME))),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor::BLACK,
            TextLayout::new_with_justify(Justify::Center),
        ));
}