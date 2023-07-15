use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
#[allow(dead_code)]
pub struct Health {
    pub current: u32,
    pub max: u32
}

#[allow(dead_code)]
impl Health {
    pub fn new(health: u32) -> Self {
        Self {
            current: health,
            max: health
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn heal(&mut self, amount: u32) {
        self.current += amount;
        self.current = self.current.clamp(0, self.max);
    }

    pub fn damage(&mut self, amount: u32) {
        self.current -= amount;
        self.current = self.current.clamp(0, self.max);
    }
}