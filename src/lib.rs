use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, window::PrimaryWindow, math::vec2};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
struct Bullet(Entity); // entity is the shooter

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Resource)]
struct CursorPosition(Vec2);

pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: format!("disarray: alpha"),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_startup_system(setup)
    .add_systems((
        player_input,
        move_vel,
        update_cursor_pos,
        player_shoot,
        player_other_shoot,
        player_rotation,
    ).chain())
    .insert_resource(CursorPosition(Vec2::ZERO))
    .insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera_bundle = Camera2dBundle::default();
    commands.spawn((
        camera_bundle,
        PlayerCamera
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("hakr.png"),
            ..default()
        },
        Player,
        Velocity(Vec2::ZERO)
    ));
}

fn player_input(
    mut player_query: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>
) {
    let mut player_velocity = player_query.single_mut();
    player_velocity.0 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        player_velocity.0.x = -200.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        player_velocity.0.x = 200.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        player_velocity.0.y = 200.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        player_velocity.0.y = -200.0;
    }
}

fn move_vel(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    for (mut player_transform, player_velocity) in query.iter_mut() {   
        player_transform.translation.x += player_velocity.0.x * delta;
        player_transform.translation.y += player_velocity.0.y * delta;
    }
}

fn update_cursor_pos(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mut cursor_pos_res: ResMut<CursorPosition>
) {
    let Some(cursor_pos) = cursor_world_pos(&window_query, &camera_query) else { return; };
    cursor_pos_res.0 = cursor_pos;
}

fn player_shoot(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    cursor_pos: Res<CursorPosition>
) {
    if !buttons.just_pressed(MouseButton::Left) { return }

    let (player_entity, player_transform) = player_query.single();
    let player_pos = player_transform.translation.truncate();
    let diff_pos = (cursor_pos.0 - player_pos).normalize();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_transform.translation),
            texture: asset_server.load("zero.png"),
            ..default()
        },
        Bullet(player_entity),
        Velocity(vec2(diff_pos.x * 300.0, diff_pos.y * 300.0))
    ));
}

fn player_other_shoot(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if !buttons.just_pressed(MouseButton::Right) { return }

    let (player_entity, player_transform) = player_query.single();

    for deg in (0..360).step_by(10) {
        let rad = deg as f32 * 3.141592 / 180.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(player_transform.translation),
                texture: asset_server.load("zero.png"),
                ..default()
            },
            Bullet(player_entity),
            Velocity(vec2(rad.sin() * 300.0, rad.cos() * 300.0))
        ));
    }
}

fn player_rotation(
    mut player_query: Query<(&mut Transform, &mut Sprite), With<Player>>,
    cursor_pos: Res<CursorPosition>
) {
    let (mut player_transform, mut player_sprite) = player_query.single_mut();
    let player_pos = player_transform.translation.truncate();

    let diff_pos = (cursor_pos.0 - player_pos).normalize();
    let angle = diff_pos.y.atan2(diff_pos.x);
    player_transform.rotation = Quat::from_rotation_z(angle);

    info!("{}", angle);
    player_sprite.flip_y = diff_pos.x < 0.0;
}

fn cursor_world_pos(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera_query.single();
    let Ok(primary) = window_query.get_single() else { return None };

    primary
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}