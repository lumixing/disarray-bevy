use bevy::prelude::*;

use crate::{physics::components::{Velocity, CursorPosition}, bullet::components::SpawnBulletEvent};

use super::components::*;

pub fn spawn_player_camera(
    mut commands: Commands
) {
    let camera_bundle = Camera2dBundle::default();
    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("hakr.png"),
            ..default()
        },
        Player,
        Velocity(Vec2::ZERO),
        GunType::Pistol.new()
    ));
}

pub fn handle_movement(
    mut player_query: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut player_velocity = player_query.single_mut();
    let mut move_dir = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        move_dir.x = -1.0;
    } if keyboard_input.pressed(KeyCode::D) {
        move_dir.x = 1.0;
    } if keyboard_input.pressed(KeyCode::W) {
        move_dir.y = 1.0;
    } if keyboard_input.pressed(KeyCode::S) {
        move_dir.y = -1.0;
    }

    move_dir = move_dir.normalize_or_zero();
    player_velocity.0 += move_dir * 100.0;
    player_velocity.0 = player_velocity.0.clamp_length_max(2000.0);
}

pub fn handle_shooting(
    player_query: Query<(Entity, &Transform, &Gun), With<Player>>,
    buttons: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
    mut event: EventWriter<SpawnBulletEvent>
) {
    if !(buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right)) { return; }

    let (player_entity, player_transform, player_gun) = player_query.single();
    let player_pos = player_transform.translation.truncate();
    let diff_pos = (cursor_pos.0 - player_pos).normalize();

    event.send(SpawnBulletEvent {
        position: player_pos,
        direction: diff_pos,
        gun_stats: player_gun.gun_stats,
        owner: player_entity,
        is_primary: buttons.just_pressed(MouseButton::Left)
    });
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
    camera_transform.translation.z = 999.9;
}