use bevy::prelude::*;

mod systems;
pub(crate) mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            systems::spawn_player_camera,
            systems::spawn_player,
        ));

        app.add_systems(Update, (
            systems::handle_movement,
            systems::handle_shooting,
            systems::camera_follow_player
        ));
    }
}