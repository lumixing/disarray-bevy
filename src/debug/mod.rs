use bevy::prelude::*;

mod systems;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            systems::spawn_text, 
        ));

        app.add_systems(Update, (
            systems::update_text,
            systems::undraw_health,
            systems::draw_health
        ).chain());
    }
}