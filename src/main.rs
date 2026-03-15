mod models;
mod physics;
mod render;

use macroquad::prelude::*;
use models::ballon::Ballon;

#[macroquad::main("Test Ballon - Head Soccer")]
async fn main() {

    let t_stade = load_texture("src/assets/stade/stade.png").await.unwrap();
    let t_ballon = load_texture("src/assets/ballon/ballon.png").await.unwrap();

    // Initialisation du ballon au milieu de l'écran
    let mut ballon = Ballon::new(screen_width() / 2.0, 100.0, 30.0, t_ballon);

    loop {
        // Pour tester le rebond, on peut appliquer une impulsion au ballon lorsque la barre d'espace est pressée
        if is_key_pressed(KeyCode::Space) {
            ballon.vy = -15.0; // Impulsion vers le haut
            ballon.vx = 8.0;   // Impulsion vers la droite
        }
      
        physics::appliquer_physique_ballon(&mut ballon);
        render::dessiner_tout(&ballon, &t_stade);

        next_frame().await
    }
}