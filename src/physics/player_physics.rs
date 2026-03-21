// src/physics.rs
use crate::models::joueur::Joueur;
use macroquad::prelude::screen_width;
use crate::physics::{GRAVITE_JOUEUR_REFERENCE, niveau_sol};


pub fn appliquer_physique(joueur: &mut Joueur) {
    let sol_y = niveau_sol();
    let y_sol_joueur = joueur.y_pose_au_sol(sol_y);

    joueur.x += joueur.vx;
    joueur.y += joueur.vy;
    
    // gravité joueur
    joueur.vy += GRAVITE_JOUEUR_REFERENCE; 
    
    // -- COLLISIONS SOL --
    if joueur.y > y_sol_joueur { 
        joueur.y = y_sol_joueur; 
        joueur.vy = 0.0; // On arrête la chute quand on touche le sol
        joueur.nb_sauts = 0; // Réinitialise le nombre de sauts disponibles
    }

    // --- COLLISION BORD GAUCHE (x = 0) ---
    if joueur.x < 0.0 {
        joueur.x = 0.0;
        joueur.vx = 0.0; // On arrête le mouvement horizontal
    }

    // --- COLLISION BORD DROIT (Largeur de l'écran) ---
    let largeur_joueur = joueur.largeur_collision();
    if joueur.x > screen_width() - largeur_joueur {
        joueur.x = screen_width() - largeur_joueur;
        joueur.vx = 0.0;
    }

    // --- COLLISION HAUT DE L'ÉCRAN ---
    if joueur.y < 0.0 {
        joueur.y = 0.0;
        joueur.vy = 0.0;
    }

}