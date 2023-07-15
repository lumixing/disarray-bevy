use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use rand::prelude::*;

use crate::{physics::components::Velocity, player::components::GunType, enemy::components::{Enemy, Health}};

use super::components::{SpawnBulletEvent, Bullet, BulletBounce};

pub fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<SpawnBulletEvent>,
) {
    for ev in event.iter() {
        match ev.gun_stats.bullet_type {
            GunType::Pistol => shoot_pistol_bullet(&mut commands, ev, &asset_server),
            GunType::Shotgun => shoot_shotgun_bullet(&mut commands, ev, &asset_server),
        }
    }
}

fn shoot_pistol_bullet(
    commands: &mut Commands,
    ev: &SpawnBulletEvent,
    asset_server: &Res<AssetServer>, // Handle<> pls
) {
    if ev.is_primary {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(ev.position.extend(0.0)),
                texture: asset_server.load("zero.png"),
                ..default()
            },
            Bullet {},
            Velocity(Vec2 {
                x: ev.direction.x * 300.0,
                y: ev.direction.y * 300.0
            })
        ));
    } else {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(ev.position.extend(0.0)),
                texture: asset_server.load("one.png"),
                ..default()
            },
            Bullet {},
            BulletBounce,
            Velocity(Vec2 {
                x: ev.direction.x * 300.0,
                y: ev.direction.y * 300.0
            })
        ));
    }
}

fn shoot_shotgun_bullet(
    commands: &mut Commands,
    ev: &SpawnBulletEvent,
    asset_server: &Res<AssetServer>, // Handle<> pls
) {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let velocity = rng.gen_range(400.0..500.0);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(ev.position.extend(0.0)),
                texture: asset_server.load("zero.png"),
                ..default()
            },
            Bullet {},
            Velocity(Vec2 {
                x: (ev.direction.x + rng.gen_range(-0.1..0.1)) * velocity,
                y: (ev.direction.y + rng.gen_range(-0.1..0.1)) * velocity
            })
        ));
    }
}

pub fn check_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Bullet>)>
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, enemy_transform, mut entity_health) in enemy_query.iter_mut() {
            let Some(_collision) = collide(
                bullet_transform.translation,
                Vec2::splat(16.),
                enemy_transform.translation,
                Vec2::splat(32.),
            ) else { continue };

            info!("colliding bullet with enemy! woohoo >:D");

            commands.entity(bullet_entity).despawn();
            
            entity_health.damage(10);
            if entity_health.is_dead() {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}