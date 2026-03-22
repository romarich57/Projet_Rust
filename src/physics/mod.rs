pub mod ball_physics;
pub mod player_physics;
pub mod collision;

use macroquad::prelude::{screen_height, screen_width};

pub const REFERENCE_WIDTH: f32 = 1000.0;
pub const REFERENCE_HEIGHT: f32 = 600.0;

pub const GROUND_Y_REFERENCE: f32 = 420.0;
pub const PLAYER_GRAVITY_REFERENCE: f32 = 0.5;
pub const BALL_GRAVITY_REFERENCE: f32 = 0.2;

pub const GOAL_MARGIN_REFERENCE: f32 = 110.0;
pub const CROSSBAR_Y_REFERENCE: f32 = 200.0;
pub const CROSSBAR_THICKNESS_REFERENCE: f32 = 15.0;

pub fn scale_x() -> f32 {
	screen_width() / REFERENCE_WIDTH
}

pub fn scale_y() -> f32 {
	screen_height() / REFERENCE_HEIGHT
}

pub fn ground_level() -> f32 {
	GROUND_Y_REFERENCE * scale_y()
}