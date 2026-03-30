pub mod ball_physics;
pub mod collision;
pub mod player_physics;

use macroquad::prelude::{screen_height, screen_width};

pub const REFERENCE_WIDTH: f32 = 1000.0;
pub const REFERENCE_HEIGHT: f32 = 600.0;

pub const PLAYER_GRAVITY_REFERENCE: f32 = 0.5;
pub const BALL_GRAVITY_REFERENCE: f32 = 0.3;

pub fn scale_x() -> f32 {
    screen_width() / REFERENCE_WIDTH
}

pub fn scale_y() -> f32 {
    screen_height() / REFERENCE_HEIGHT
}
