use macroquad::prelude::*;
use crate::models::joueur::Joueur;

pub fn dessiner_tout(joueur: &Joueur, tex_stade: &Texture2D) {
    // 1. Dessiner le stade (toujours en premier)
    draw_texture_ex(tex_stade, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    });

    // 2. Dessiner le pied REDIMENSIONNÉ
    draw_texture_ex(
        &joueur.texture_pied,
        joueur.x,
        joueur.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(250.0, 120.0)), //(largeur, hauteur)
            rotation: joueur.angle_pied,
            ..Default::default()
        },
    );

    // 3. Dessiner la tête REDIMENSIONNÉE
    // On réduit la taille et on ajuste le décalage (y - 50.0 par exemple)
    draw_texture_ex(
        &joueur.texture_tete,
        joueur.x + 15.0, // Petit décalage X pour centrer la tête sur le pied
        joueur.y - 110.0, // Décalage Y pour poser la tête AU-DESSUS du pied
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(270.0, 200.0)), // <-- Ajuster la taille ici
            ..Default::default()
        },
    );
}