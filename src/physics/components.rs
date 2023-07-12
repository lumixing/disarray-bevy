use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);