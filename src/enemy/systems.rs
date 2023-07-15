use bevy::prelude::*;

use super::components::*;

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("warning.png"),
            transform: Transform::from_translation(Vec3::splat(64.)),
            ..default()
        },
        Enemy,
        Health::new(100)
    ));
}