pub mod ball_physics;
pub mod player_physics;
pub mod collision;

use macroquad::prelude::{screen_height, screen_width};

pub const LARGEUR_REFERENCE: f32 = 1000.0;
pub const HAUTEUR_REFERENCE: f32 = 600.0;

pub const SOL_Y_REFERENCE: f32 = 420.0;
pub const GRAVITE_JOUEUR_REFERENCE: f32 = 0.5;
pub const GRAVITE_BALLON_REFERENCE: f32 = 0.2;

pub const POTEAU_MARGE_REFERENCE: f32 = 110.0;
pub const BARRE_Y_REFERENCE: f32 = 200.0;
pub const BARRE_EPAISSEUR_REFERENCE: f32 = 15.0; 

pub fn echelle_x() -> f32 {
	screen_width() / LARGEUR_REFERENCE
}

pub fn echelle_y() -> f32 {
	screen_height() / HAUTEUR_REFERENCE
}

pub fn niveau_sol() -> f32 {
	SOL_Y_REFERENCE * echelle_y()
}