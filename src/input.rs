use macroquad::prelude::*;
use crate::models::joueur::Joueur;

pub fn gerer_clavier(joueur: &mut Joueur) { //&mut Joueur : on passe une référence mutable à la fonction pour pouvoir modifier les propriétés du joueur (comme sa vitesse et son état de tir) en fonction des entrées clavier. Cela permet de mettre à jour l'état du joueur en temps réel pendant le jeu.
    let vitesse = 5.0;

    // Gauche / Droite
    if is_key_down(KeyCode::Left) {
        joueur.vx = -vitesse;
    } else if is_key_down(KeyCode::Right) {
        joueur.vx = vitesse;
    } else {
        joueur.vx = 0.0;
    }

    // Saut
    if is_key_pressed(KeyCode::Up) && joueur.nb_sauts < 2 {
        joueur.vy = if joueur.nb_sauts == 0 { -10.0 } else { -8.0 };  
        joueur.nb_sauts += 1; // Incrémente le nombre de sauts effectués
    }

    // Gestion du tir (Espace)
    if is_key_pressed(KeyCode::Space) {
        joueur.en_tir = true;
    }
}

pub fn update_animations(joueur: &mut Joueur) {
    if joueur.en_tir {
        joueur.angle_pied -= 0.2; // Le pied se lève
        if joueur.angle_pied < -1.0 { // Limite de l'angle
            joueur.en_tir = false;
        }
    } else if joueur.angle_pied < 0.0 {
        joueur.angle_pied += 0.1; // Le pied redescend tout seul
    }
}