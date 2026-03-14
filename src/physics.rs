// src/physics.rs
use crate::models::joueur::Joueur;
use macroquad::prelude::screen_width;


pub fn appliquer_physique(joueur: &mut Joueur) {

    joueur.x += joueur.vx;
    joueur.y += joueur.vy;
    
    // gravité joueur
    joueur.vy += 0.25; 
    
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

}