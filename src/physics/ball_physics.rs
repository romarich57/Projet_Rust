use crate::models::ballon::Ballon;
use macroquad::prelude::screen_width;
use crate::physics::{
    BARRE_EPAISSEUR_REFERENCE, BARRE_Y_REFERENCE, GRAVITE_BALLON_REFERENCE,
    POTEAU_MARGE_REFERENCE, echelle_x, echelle_y, niveau_sol,
};

pub fn appliquer_physique_ballon(ballon: &mut Ballon) {
    let niveau_sol = niveau_sol();
    let poteau_gauche_x = POTEAU_MARGE_REFERENCE * echelle_x();
    let poteau_droit_x = screen_width() - POTEAU_MARGE_REFERENCE * echelle_x();
    let barre_y = BARRE_Y_REFERENCE * echelle_y();
    let epaisseur = BARRE_EPAISSEUR_REFERENCE * echelle_y();

    // Gravité
    ballon.vy += GRAVITE_BALLON_REFERENCE;

    // Mise à jour de la position avec la vélocité
    ballon.x += ballon.vx;
    ballon.y += ballon.vy;

    ballon.angle += ballon.vx * 0.05;

    // utiliser la hitbox du ballon pour toutes les vérifications physiques
    let (bcx, bcy, bcr) = ballon.hitbox_cercle();

    if bcx < poteau_gauche_x || bcx > poteau_droit_x {
        // rebond par dessus 
        if bcy + bcr > barre_y && bcy < barre_y && ballon.vy > 0.0 {
            ballon.y = barre_y - bcr - ballon.hitbox.offset_y;
            ballon.vy = -ballon.vy * 0.8; // Rebond vers le haut
            ballon.vx *= 0.98; // Légère friction sur le métal
        }
        // Rebond par dessous
        else if bcy - bcr < barre_y + epaisseur && bcy > barre_y && ballon.vy < 0.0 {
            ballon.y = barre_y + epaisseur + bcr - ballon.hitbox.offset_y;
            ballon.vy = -ballon.vy * 0.8; // Rebond vers le bas
        }
    }


    // Si le ballon est rentré dans la cage (derrière la ligne de but et sous la barre)
    if (bcx < poteau_gauche_x || bcx > poteau_droit_x) && bcy > barre_y {
        ballon.vx *= 0.93; // Le filet freine le ballon 
    }

    // Rebond au sol seulement si la balle tombe vraiment d'en haut
    // Sinon elle se pose sans rebond artificiel
    if bcy + bcr > niveau_sol {
        ballon.y = niveau_sol - bcr - ballon.hitbox.offset_y;

        if ballon.vy > 0.0 {
            let vitesse_impact = ballon.vy;
            let seuil_rebond = 1.1 * echelle_y();

            if vitesse_impact > seuil_rebond {
                // Perte d'energie a chaque rebond pour qu'elle finisse par s'arreter
                ballon.vy = -vitesse_impact * 0.62;
            } else {
                ballon.vy = 0.0;
            }
        } else {
            ballon.vy = 0.0;
        }

        // Frottement au sol pour arreter progressivement le mouvement horizontal
        ballon.vx *= 0.94;
        if ballon.vx.abs() < 0.03 * echelle_x() {
            ballon.vx = 0.0;
        }
    }

    // Retour progressif de la balle vers le terrain quand elle approche les bordures
    let zone_retour = 120.0 * echelle_x();
    let force_retour = 0.18 * echelle_x();

    // Bordure gauche: on empeche la sortie franche puis on pousse doucement doucement vers la droite
    if bcx - bcr < 0.0 {
        ballon.x = bcr - ballon.hitbox.offset_x;
        if ballon.vx < 0.0 {
            ballon.vx = 0.0;
        }
    }
    let distance_gauche = (bcx - bcr).max(0.0);
    if distance_gauche < zone_retour {
        let intensite = 1.0 - distance_gauche / zone_retour;
        ballon.vx += force_retour * intensite;
    }

    // Bordure droite : pareil que a gauche
    if bcx + bcr > screen_width() {
        ballon.x = screen_width() - bcr - ballon.hitbox.offset_x;
        if ballon.vx > 0.0 {
            ballon.vx = 0.0;
        }
    }
    let distance_droite = (screen_width() - (bcx + bcr)).max(0.0);
    if distance_droite < zone_retour {
        let intensite = 1.0 - distance_droite / zone_retour;
        ballon.vx -= force_retour * intensite;
    }

    // Collision avec le plafond
    if bcy - bcr < 0.0 { // 0.0 représente le plafond
        ballon.y = bcr + ballon.hitbox.offset_y;
        ballon.vy = -ballon.vy * 0.6;
    }
}