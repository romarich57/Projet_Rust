use macroquad::prelude::*;

pub struct Ballon {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub rayon_visuel: f32,
    pub hitbox: HitboxCircle,
    pub angle: f32,
    pub texture: Texture2D,
}

impl Ballon {
    pub fn new(x: f32, y: f32, rayon_visuel: f32, texture: Texture2D) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            rayon_visuel,
            hitbox: HitboxCircle {
                offset_x: 0.0,
                offset_y: 0.0,
                rayon: rayon_visuel,
            },
            angle: 0.0,
            texture,
        }
    }

    pub fn rayon_visuel(&self) -> f32 {
        self.rayon_visuel
    }

    pub fn hitbox_cercle(&self) -> (f32, f32, f32) {
        (
            self.x + self.hitbox.offset_x,
            self.y + self.hitbox.offset_y,
            self.hitbox.rayon,
        )
    }

    pub fn set_hitbox_cercle(&mut self, offset_x: f32, offset_y: f32, rayon: f32) {
        self.hitbox.offset_x = offset_x;
        self.hitbox.offset_y = offset_y;
        self.hitbox.rayon = rayon;
    }
}

pub struct HitboxCircle {
    pub offset_x: f32,
    pub offset_y: f32,
    pub rayon: f32,
}