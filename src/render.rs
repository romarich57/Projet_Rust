use macroquad::prelude::*;
use crate::models::ballon::Ballon;
use crate::models::joueur::Joueur;

pub fn dessiner_tout(joueur: &Joueur, tex_stade: &Texture2D, ballon: &Ballon, debug_hitbox: bool) {
    // 1. Dessiner le stade (toujours en premier)
    draw_texture_ex(tex_stade, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    });

    // Dessiner le ballon avec sa rotation
    draw_texture_ex(
        &ballon.texture,
        ballon.x - ballon.rayon_visuel(), 
        ballon.y - ballon.rayon_visuel(),
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(ballon.rayon_visuel() * 2.0, ballon.rayon_visuel() * 2.0)),
            rotation: ballon.angle,
            ..Default::default()
        },
    );
    // 2. Dessiner le pied REDIMENSIONNÉ
    draw_texture_ex(
        &joueur.texture_pied,
        joueur.x,
        joueur.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(joueur.largeur_pied, joueur.hauteur_pied)),
            rotation: joueur.angle_pied,
            ..Default::default()
        },
    );

    // 3. Dessiner la tête REDIMENSIONNÉE
    // On réduit la taille et on ajuste le décalage (y - 50.0 par exemple)
    draw_texture_ex(
        &joueur.texture_tete,
        joueur.x + joueur.offset_tete_x,
        joueur.y + joueur.offset_tete_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(joueur.largeur_tete, joueur.hauteur_tete)),
            ..Default::default()
        },
    );

    if debug_hitbox {
        dessiner_hitbox_debug(joueur, ballon);
    }
}

fn dessiner_hitbox_debug(joueur: &Joueur, ballon: &Ballon) {
    let (pied_x, pied_y, pied_l, pied_h) = joueur.hitbox_rect_pied();
    let (tete_x, tete_y, tete_l, tete_h) = joueur.hitbox_rect_tete();

    let ballon_hit = ballon.hitbox_cercle();
    let ballon_cx = ballon_hit.0;
    let ballon_cy = ballon_hit.1;
    let ballon_r = ballon_hit.2;

    draw_rectangle_lines(pied_x, pied_y, pied_l, pied_h, 2.0, RED);
    draw_rectangle_lines(tete_x, tete_y, tete_l, tete_h, 2.0, ORANGE);
    draw_rectangle_lines(
        ballon_cx - ballon_r,
        ballon_cy - ballon_r,
        ballon_r * 2.0,
        ballon_r * 2.0,
        2.0,
        LIME,
    );

    draw_text("DEBUG HITBOX (Y)", 15.0, 30.0, 26.0, YELLOW);
}