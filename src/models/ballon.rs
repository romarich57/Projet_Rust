use macroquad::prelude::*;

pub struct Ballon {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub rayon: f32,
    pub angle: f32,
    pub texture: Texture2D,
}

impl Ballon {
    pub fn new(x: f32, y: f32, rayon: f32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            rayon,
            angle: 0.0,
            texture,
        }
    }
}