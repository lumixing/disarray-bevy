use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gun {
    pub gun_stats: GunStats,
    pub bullets: u32,
    // reload_timer, secondary_timer
}

#[derive(Clone, Copy)]
pub struct GunStats {
    // pub name: String,
    pub bullet_type: GunType,
    pub bullet_count: u32,
    pub critical_chance: f32,
    // pub primary_bullet_components: Vec<Component>
}

#[derive(Clone, Copy)]
pub enum GunType {
    Pistol,
    Shotgun
}

impl GunType {
    pub fn new(&self) -> Gun {
        match self {
            GunType::Pistol => Gun {
                gun_stats: GunStats {
                    // name: "Pistol".to_string(),
                    bullet_type: Self::Pistol,
                    bullet_count: 6,
                    critical_chance: 0.1
                },
                bullets: 6
            },
            GunType::Shotgun => Gun {
                gun_stats: GunStats {
                    // name: "Shotgun".to_string(),
                    bullet_type: Self::Shotgun,
                    bullet_count: 12,
                    critical_chance: 0.1
                },
                bullets: 12
            },
        }
    }
}