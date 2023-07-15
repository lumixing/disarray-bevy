use bevy::prelude::*;

use self::components::SpawnBulletEvent;

mod systems;
pub(crate) mod components;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            systems::spawn_bullet, 
            systems::check_collision
        ));

        app.add_event::<SpawnBulletEvent>();
    }
}