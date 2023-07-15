use bevy::prelude::*;

use crate::player::components::GunStats;

#[derive(Component)]
#[allow(dead_code)]
pub struct Bullet {
    // pub owner: Entity,
    // pub lifespan: Timer,
    // pub gun_type: GunType
}

#[derive(Event)]
pub struct SpawnBulletEvent {
    pub position: Vec2,
    pub direction: Vec2,
    pub gun_stats: GunStats,
    pub owner: Entity,
    pub is_primary: bool
}

#[derive(Component)]
pub struct BulletBounce;