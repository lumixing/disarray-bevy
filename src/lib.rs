use bevy::prelude::*;

mod player;
mod physics;
mod bullet;

pub fn app() -> App {
    let mut app = App::new();
    
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    mode: bevy::window::WindowMode::Windowed,
                    title: format!("disarray.rs"),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins((
        player::PlayerPlugin,
        physics::PhysicsPlugin,
        bullet::BulletPlugin,
    ));

    app.add_systems(Update, bevy::window::close_on_esc);

    app
}