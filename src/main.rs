mod models;
mod input;
mod render;

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

        // 2. Mise à jour des positions (On bouge le joueur)
        joueur.x += joueur.vx;
        joueur.y += joueur.vy;
        
        // Petite friction pour que le saut s'arrête (très basique)
        joueur.vy += 0.25; 
        *
        // -- COLLISIONS SOL --
        if joueur.y > 580.0 { 
            joueur.y = 580.0; 
            joueur.vy = 0.0; // On arrête la chute quand on touche le sol
        }

        // --- COLLISION BORD GAUCHE (x = 0) ---
        if joueur.x < 0.0 {
            joueur.x = 0.0;
            joueur.vx = 0.0; // On arrête le mouvement horizontal
        }

        // --- COLLISION BORD DROIT (Largeur de l'écran) ---
        // On soustrait environ 80.0 (la largeur de ton joueur) pour ne pas qu'il dépasse
        let largeur_joueur = 200.0; 
        if joueur.x > screen_width() - largeur_joueur {
            joueur.x = screen_width() - largeur_joueur;
            joueur.vx = 0.0;
        }

        // --- COLLISION HAUT DE L'ÉCRAN ---
        if joueur.y < 0.0 {
            joueur.y = 0.0;
            joueur.vy = 0.0;
        }

        // 3. Rendu
        render::dessiner_tout(&joueur, &t_stade);

        next_frame().await
    }
}