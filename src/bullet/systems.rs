use std::time::Duration;

use bevy::{prelude::*, math::vec2};

use crate::physics::components::Velocity;

use super::components::{SpawnBulletEvent, Bullet};

pub fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<SpawnBulletEvent>
) {
    for ev in event.iter() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(ev.position.extend(0.0)),
                texture: asset_server.load("zero.png"),
                ..default()
            },
            Bullet {
                // owner: player_entity,
                lifespan: Timer::new(Duration::from_secs(5), TimerMode::Once)
            },
            Velocity(vec2(ev.direction.x * 300.0, ev.direction.y * 300.0))
        ));
    }
}