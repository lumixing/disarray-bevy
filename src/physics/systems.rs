use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::components::{PlayerCamera, Player};

use super::components::{Velocity, CursorPosition};

pub fn update_player_translation(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    
    for (mut transform, mut velocity) in query.iter_mut() {   
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
        velocity.0 = velocity.0.lerp(Vec2::ZERO, 0.2);
    }
}

pub fn update_translation(
    mut query: Query<(&mut Transform, &Velocity), Without<Player>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    
    for (mut transform, velocity) in query.iter_mut() {   
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
    }
}

pub fn update_cursor_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mut cursor_pos: ResMut<CursorPosition>
) {
    let (camera, camera_transform) = camera_query.single();
    let Ok(primary) = window_query.get_single() else { return; };

    let Some(pos) = primary
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) else { return; };
    cursor_pos.0 = pos;
}