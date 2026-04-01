use macroquad::prelude::*;

pub struct Ball {
    pub x: f32, // horizontal du ballon
    pub y: f32, // vertical du ballon
    pub vx: f32, // vitesse horizontale du ballon
    pub vy: f32, // vitesse verticale du ballon
    pub visual_radius: f32, // rayon visuel du ballon
    pub hitbox: HitboxCircle, // hitbox circulaire du ballon
    pub angle: f32, // angle de rotation du ballon (effet spin)
    pub texture: Texture2D, // texture du ballon
}

impl Ball {
    pub fn new(x: f32, y: f32, visual_radius: f32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            visual_radius,
            hitbox: HitboxCircle {
                offset_x: 0.0, // pas de décalage par défaut, le hitbox est centré sur la position du ballon
                offset_y: 0.0,
                radius: visual_radius,
            },
            angle: 0.0, // pas de rotation initiale
            texture,
        }
    }

    pub fn visual_radius(&self) -> f32 {
        self.visual_radius
    }

    pub fn circle_hitbox(&self) -> (f32, f32, f32) {
        (
            self.x + self.hitbox.offset_x,
            self.y + self.hitbox.offset_y,
            self.hitbox.radius,
        )
    }

    // permet de configurer la hitbox circulaire du ballon, en cas de besoin de décalage ou de changement de rayon
    pub fn set_circle_hitbox(&mut self, offset_x: f32, offset_y: f32, radius: f32) {
        self.hitbox.offset_x = offset_x;
        self.hitbox.offset_y = offset_y;
        self.hitbox.radius = radius;
    }
}

pub struct HitboxCircle {
    pub offset_x: f32,
    pub offset_y: f32,
    pub radius: f32,
}
