use crate::models::ballon::Ballon;
use macroquad::prelude::screen_width;

pub fn appliquer_physique_ballon(ballon: &mut Ballon) {
    let niveau_sol = 410.0; 
    let poteau_gauche_x = 110.0; // Limite du poteau gauche
    let poteau_droit_x = screen_width() - 110.0; // Limite du poteau droit
    let barre_y = 200.0; // Hauteur de la barre transversale
    let epaisseur = 15.0; // L'épaisseur de la barre


    // Gravité
    ballon.vy += 0.2; 
    
    // Mise à jour de la position avec la vélocité
    ballon.x += ballon.vx;
    ballon.y += ballon.vy;
    
    
    ballon.angle += ballon.vx * 0.05; 

    if ballon.x < poteau_gauche_x || ballon.x > poteau_droit_x {
        // rebond par dessus 
        if ballon.y + ballon.rayon > barre_y && ballon.y < barre_y && ballon.vy > 0.0 {
            ballon.y = barre_y - ballon.rayon;
            ballon.vy = -ballon.vy * 0.8; // Rebond vers le haut
            ballon.vx *= 0.98; // Légère friction sur le métal
        }
        // Rebond par dessous
        else if ballon.y - ballon.rayon < barre_y + epaisseur && ballon.y > barre_y && ballon.vy < 0.0 {
            ballon.y = barre_y + epaisseur + ballon.rayon;
            ballon.vy = -ballon.vy * 0.8; // Rebond vers le bas
        }
    }


    // Si le ballon est rentré dans la cage (derrière la ligne de but et sous la barre)
    if (ballon.x < poteau_gauche_x || ballon.x > poteau_droit_x) && ballon.y > barre_y {
        ballon.vx *= 0.93; // Le filet freine le ballon 
    }

   // on consider que le sol est à y = 580
    if ballon.y + ballon.rayon > niveau_sol { 
        ballon.y = niveau_sol - ballon.rayon;
        ballon.vy = -ballon.vy * 0.75; // Rebond : perte de vitesse
        ballon.vx *= 0.98; // Friction : légère perte de vitesse horizontale
    }

   // Gestion des collisions avec les murs gauche et droite

   // Mur gauche
    if ballon.x - ballon.rayon < 0.0 {
        ballon.x = ballon.rayon;
        ballon.vx = -ballon.vx * 0.4;
    }

    // Mur droit
    if ballon.x + ballon.rayon > screen_width() {
        ballon.x = screen_width() - ballon.rayon;
        ballon.vx = -ballon.vx * 0.4;
    }

    // Collision avec le plafond
    if ballon.y - ballon.rayon < 0.0 { // 0.0 représente le plafond
        ballon.y = ballon.rayon;
        ballon.vy = -ballon.vy * 0.6;
    }
}