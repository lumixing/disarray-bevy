use bevy::prelude::*;

use self::components::CursorPosition;

mod systems;
pub(crate) mod components;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            systems::update_player_translation,
            systems::update_translation,
            systems::update_cursor_position,
        ));

        app.insert_resource(CursorPosition(Vec2::ZERO));
    }
}