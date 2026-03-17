mod models;
mod render;
mod physics;
mod input;

use macroquad::prelude::*;
use models::ballon::Ballon;
use physics::player_physics;
use models::joueur::Joueur;



fn configuration_fenetre() -> Conf {
    Conf {
        window_title: "Test Ballon - Head Soccer".to_owned(),
        window_width: 1000,      
        window_height: 600,     
        window_resizable: false, // Empêche la redimension de la fenêtre
        ..Default::default()
    }
}

#[macroquad::main(configuration_fenetre())]
async fn main() {

    let t_ballon = load_texture("src/assets/ballon/ballon.png").await.unwrap();

    // Initialisation du ballon au milieu de l'écran
    let mut ballon = Ballon::new(screen_width() / 2.0, 100.0, 30.0, t_ballon);

    // Chargement des textures (indispensable au début)
    let t_stade = load_texture("src/assets/stade/stade.png").await.unwrap(); //await pour attendre que la texture soit chargée avant de continuer. unwrap() pour gérer les erreurs de chargement (ici on panique si ça échoue, mais en vrai il faudrait mieux gérer ça).
    let t_tete = load_texture("src/assets/joueur/tete.png").await.unwrap(); //unwrap() est une méthode qui permet de récupérer la valeur contenue dans un Result ou Option. Si le résultat est Err ou None, unwrap() fera paniquer le programme. C'est une manière rapide de gérer les erreurs, mais dans un vrai projet, il serait préférable d'utiliser une gestion d'erreur plus robuste.
    let t_pied = load_texture("src/assets/joueur/pied.png").await.unwrap();

    let mut joueur = Joueur::new(600.0, 580.0, t_tete, t_pied);

    loop {
        // Pour tester le rebond, on peut appliquer une impulsion au ballon lorsque la barre d'espace est pressée
        if is_key_pressed(KeyCode::Space) {
            ballon.vy = -15.0; // Impulsion vers le haut
            ballon.vx = 8.0;   // Impulsion vers la droite
        }

        input::gerer_clavier(&mut joueur);
        input::update_animations(&mut joueur);

        player_physics::appliquer_physique(&mut joueur);

        
        render::dessiner_tout(&joueur, &t_stade, &ballon);
        physics::ball_physics::appliquer_physique_ballon(&mut ballon);
        render::dessiner_tout(&joueur, &t_stade, &ballon);

        next_frame().await
    }
}