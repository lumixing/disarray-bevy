use bevy::prelude::*;

use crate::{bullet::components::Bullet, physics::components::CursorPosition, player::components::Player, enemy::components::Health};

#[derive(Component)]
pub struct DebugHealth;

#[derive(Component)]
pub struct DebugText;

pub fn spawn_text(
    mut commands: Commands
) {
    commands.spawn((TextBundle::from_section(
        "bullets: idk\nmousepos: idk",
        TextStyle {
            font_size: 16.0,
            ..default()
        }
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(8.0),
        left: Val::Px(8.0),
        ..default()
    }), DebugText));
}

pub fn update_text(
    mut text_query: Query<&mut Text, With<DebugText>>,
    bullet_query: Query<&Bullet>,
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>
) {
    let mut text = text_query.single_mut();

    let string_list = vec![
        format!("player pos: {}", player_query.single().translation.truncate().floor()),
        format!("mouse pos: {}", cursor_pos.0.floor()),
        format!("bullets: {}", bullet_query.iter().count()),
    ];

    text.sections[0].value = string_list.join("\n");
}

pub fn draw_colls(
    bullet_query: Query<&Transform, With<Bullet>>,
    mut gizmos: Gizmos
) {
    for transform in bullet_query.iter() {
        gizmos.rect_2d(
            transform.translation.truncate(),
            0.0,
            Vec2::splat(16.0),
            Color::GREEN
        );     
    }
}

pub fn undraw_health(
    mut commands: Commands,
    health_query: Query<Entity, With<DebugHealth>>
) {
    for entity in health_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn draw_health(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    health_query: Query<(&Health, &Transform)>
) {
    let (camera, camera_transform) = camera_query.single();
    for (health, health_transform) in health_query.iter() {  
        let Some(pos) = camera.world_to_viewport(camera_transform, health_transform.translation) else { return; };
        
        commands.spawn((TextBundle::from_section(
            format!("{}/{}", health.current, health.max),
            TextStyle {
                font_size: 16.0,
                ..default()
            }
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(pos.y),
            left: Val::Px(pos.x),
            ..default()
        }), DebugHealth));
    }
}