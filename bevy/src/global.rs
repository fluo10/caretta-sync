use bevy::{asset::uuid::Uuid, ecs::component::Component};

#[derive(Component)]
struct Id(Uuid);
