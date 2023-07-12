use bevy::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub struct Bullet {
    // owner: Entity,
    pub lifespan: Timer
}

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub position: Vec2,
    pub direction: Vec2
}