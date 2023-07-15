use bevy::prelude::*;

mod systems;
pub(crate) mod components;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            systems::spawn_enemy,
        ));
    }
}