use macroquad::prelude::*; 
// macroquad est un moteur de jeu en Rust qui fournit des fonctionnalités pour la création de jeux 2D et 3D. 
//Il offre une API simple pour gérer les graphiques, les entrées utilisateur, les sons, etc.

#[derive(Clone, Copy)]
pub struct HitboxRect {
    pub offset_x: f32,
    pub offset_y: f32,
    pub largeur: f32,
    pub hauteur: f32,
}

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
    pub largeur_pied: f32,
    pub hauteur_pied: f32,
    pub largeur_tete: f32,
    pub hauteur_tete: f32,
    pub offset_tete_x: f32,
    pub offset_tete_y: f32,
    pub hitbox_pied: HitboxRect,
    pub hitbox_tete: HitboxRect,
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
            largeur_pied: 200.0,
            hauteur_pied: 85.0,
            largeur_tete: 200.0,
            hauteur_tete: 170.0,
            offset_tete_x: 5.0,
            offset_tete_y: -95.0,
            hitbox_pied: HitboxRect {
                offset_x: 45.0,
                offset_y: 20.0,
                largeur: 120.0,
                hauteur: 55.0,
            },
            hitbox_tete: HitboxRect {
                offset_x: 55.0,
                offset_y: -90.0,
                largeur: 90.0,
                hauteur: 110.0,
            },
        }
    }

    pub fn appliquer_taille_relative_ecran(&mut self, largeur_ecran: f32, hauteur_ecran: f32) {
        let cible_largeur = largeur_ecran * 0.20;
        let cible_hauteur = hauteur_ecran * 0.20;

        let base_largeur_pied: f32 = 200.0;
        let base_hauteur_pied: f32 = 85.0;
        let base_largeur_tete: f32 = 200.0;
        let base_hauteur_tete: f32 = 170.0;
        let base_offset_tete_x: f32 = 5.0;
        let base_offset_tete_y: f32 = -95.0;

        let base_largeur_collision = base_largeur_pied.max(base_offset_tete_x + base_largeur_tete);
        let base_hauteur_totale = base_hauteur_pied - base_offset_tete_y;

        let scale = (cible_largeur / base_largeur_collision)
            .min(cible_hauteur / base_hauteur_totale);

        self.largeur_pied = base_largeur_pied * scale;
        self.hauteur_pied = base_hauteur_pied * scale;

        self.largeur_tete = base_largeur_tete * scale;
        self.hauteur_tete = base_hauteur_tete * scale;

        self.offset_tete_x = base_offset_tete_x * scale;
        self.offset_tete_y = base_offset_tete_y * scale;

        self.hitbox_pied.offset_x = 45.0 * scale;
        self.hitbox_pied.offset_y = 20.0 * scale;
        self.hitbox_pied.largeur = 120.0 * scale;
        self.hitbox_pied.hauteur = 55.0 * scale;

        self.hitbox_tete.offset_x = 55.0 * scale;
        self.hitbox_tete.offset_y = -90.0 * scale;
        self.hitbox_tete.largeur = 90.0 * scale;
        self.hitbox_tete.hauteur = 110.0 * scale;
    }

    pub fn hitbox_rect_pied(&self) -> (f32, f32, f32, f32) {
        (
            self.x + self.hitbox_pied.offset_x,
            self.y + self.hitbox_pied.offset_y,
            self.hitbox_pied.largeur,
            self.hitbox_pied.hauteur,
        )
    }

    pub fn hitbox_rect_pied_active(&self) -> (f32, f32, f32, f32) {
        let mut hx = self.x + self.hitbox_pied.offset_x;
        let mut hy = self.y + self.hitbox_pied.offset_y;
        let mut hl = self.hitbox_pied.largeur;
        let mut hh = self.hitbox_pied.hauteur;

        // Pendant le tir, on avance et on allonge un peu la hitbox du pied
        // pour suivre l'animation visuelle.
        let progression_tir = (-self.angle_pied).clamp(0.0, 1.0);
        if progression_tir > 0.05 {
            hx -= hl * 0.38 * progression_tir;
            hy -= hh * 0.20 * progression_tir;
            hl *= 1.0 + 0.28 * progression_tir;
            hh *= 1.0 + 0.12 * progression_tir;
        }

        (hx, hy, hl, hh)
    }

    pub fn hitbox_rect_tete(&self) -> (f32, f32, f32, f32) {
        (
            self.x + self.hitbox_tete.offset_x,
            self.y + self.hitbox_tete.offset_y,
            self.hitbox_tete.largeur,
            self.hitbox_tete.hauteur,
        )
    }

    pub fn set_hitbox_pied(&mut self, offset_x: f32, offset_y: f32, largeur: f32, hauteur: f32) {
        self.hitbox_pied.offset_x = offset_x;
        self.hitbox_pied.offset_y = offset_y;
        self.hitbox_pied.largeur = largeur;
        self.hitbox_pied.hauteur = hauteur;
    }

    pub fn set_hitbox_tete(&mut self, offset_x: f32, offset_y: f32, largeur: f32, hauteur: f32) {
        self.hitbox_tete.offset_x = offset_x;
        self.hitbox_tete.offset_y = offset_y;
        self.hitbox_tete.largeur = largeur;
        self.hitbox_tete.hauteur = hauteur;
    }

    pub fn largeur_collision(&self) -> f32 {
        self.largeur_pied.max(self.offset_tete_x + self.largeur_tete)
    }

    pub fn y_pose_au_sol(&self, sol_y: f32) -> f32 {
        sol_y - self.hauteur_pied + self.hauteur_pied * 0.3
    }
}