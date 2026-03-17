use macroquad::prelude::*; // macroquad est un moteur de jeu en Rust qui fournit des fonctionnalités pour la création de jeux 2D et 3D. Il offre une API simple pour gérer les graphiques, les entrées utilisateur, les sons, etc.

pub struct Joueur {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub nb_sauts: u8,
    pub angle_pied: f32,
    pub en_tir: bool,
    pub texture_tete: Texture2D,
    pub texture_pied: Texture2D,
}

impl Joueur {
    
    pub fn new(x: f32, y: f32, tex_t: Texture2D, tex_p: Texture2D) -> Self {
        Self {
            x, y,
            vx: 0.0, vy: 0.0,
            nb_sauts: 0,
            angle_pied: 0.0,
            en_tir: false,
            texture_tete: tex_t,
            texture_pied: tex_p,
        }
    }
}