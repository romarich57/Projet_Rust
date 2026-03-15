use macroquad::prelude::*;
use crate::models::ballon::Ballon;

pub fn dessiner_tout(ballon: &Ballon, tex_stade: &Texture2D) {
    draw_texture_ex(tex_stade, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    });

    // Dessiner le ballon avec sa rotation
    draw_texture_ex(
        &ballon.texture,
        ballon.x - ballon.rayon, 
        ballon.y - ballon.rayon,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(ballon.rayon * 2.0, ballon.rayon * 2.0)),
            rotation: ballon.angle,
            ..Default::default()
        },
    );
}