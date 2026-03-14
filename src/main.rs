mod models;
mod input;
mod render;
mod physics;

use macroquad::prelude::*;
use models::joueur::Joueur;


#[macroquad::main("Head Football")]
async fn main() {
    // Chargement des textures (indispensable au début)
    let t_stade = load_texture("src/assets/stade/stade.png").await.unwrap(); //await pour attendre que la texture soit chargée avant de continuer. unwrap() pour gérer les erreurs de chargement (ici on panique si ça échoue, mais en vrai il faudrait mieux gérer ça).
    let t_tete = load_texture("src/assets/joueur/tete.png").await.unwrap(); //unwrap() est une méthode qui permet de récupérer la valeur contenue dans un Result ou Option. Si le résultat est Err ou None, unwrap() fera paniquer le programme. C'est une manière rapide de gérer les erreurs, mais dans un vrai projet, il serait préférable d'utiliser une gestion d'erreur plus robuste.
    let t_pied = load_texture("src/assets/joueur/pied.png").await.unwrap();

    let mut joueur = Joueur::new(600.0, 580.0, t_tete, t_pied);

    loop {
        // 1. Entrées clavier
        input::gerer_clavier(&mut joueur);
        input::update_animations(&mut joueur);

        // 2. Physique
        physics::appliquer_physique(&mut joueur);

        // 3. Rendu
        render::dessiner_tout(&joueur, &t_stade);

        next_frame().await
    }
}