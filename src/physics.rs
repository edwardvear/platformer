use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub const GRAVITY: f32 = 9.81;

fn gravity_system() {
}
