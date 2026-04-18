use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}
#[derive(Component)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub texture: Texture2D,
    pub color: Color,
}

#[derive(Component)]
pub struct Collider {
    pub x: f32,
    pub y: f32,
}
