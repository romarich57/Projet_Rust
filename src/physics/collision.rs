use crate::models::ballon::Ballon;
use crate::models::joueur::Joueur;

/// Collision joueur <-> ballon:
/// - collision "corps/tête" (faible impulsion)
/// - collision "pied en tir" (forte impulsion)
pub fn appliquer_collision_joueur_ballon(joueur: &Joueur, ballon: &mut Ballon) {
    let (pied_x, pied_y, pied_l, pied_h) = joueur.hitbox_rect_pied();
    let (tete_x, tete_y, tete_l, tete_h) = joueur.hitbox_rect_tete();

    // 1) Collision tête-corps: petite poussée naturelle
    let (bcx, bcy, bcr) = ballon.hitbox_cercle();

    if let Some((nx, ny, penetration)) = collision_rect_cercle(
        tete_x,
        tete_y,
        tete_l,
        tete_h,
        bcx,
        bcy,
        bcr,
    ) {
        // On sépare les objets pour éviter qu'ils restent collés
        ballon.x += nx * penetration;
        ballon.y += ny * penetration;

        // Impulsion légère
        let force = 3.0;
        ballon.vx += nx * force + joueur.vx * 0.30;

        // La balle doit toujours prendre un peu de hauteur au contact.
        ballon.vy += ny * force + joueur.vy * 0.15;
        ballon.vy -= 1.2;
        if ballon.vy > -2.6 {
            ballon.vy = -2.6;
        }
    }

    // 2) Collision pied:
    // - si joueur en tir => frappe forte
    // - sinon => petite poussée
    if let Some((nx, ny, penetration)) = collision_rect_cercle(
        pied_x,
        pied_y,
        pied_l,
        pied_h,
        bcx,
        bcy,
        bcr,
    ) {
        ballon.x += nx * penetration;
        ballon.y += ny * penetration;

        let force = if joueur.en_tir { 11.5 } else { 3.2 };
        let lift = if joueur.en_tir { 2.8 } else { 1.3 };

        ballon.vx += nx * force + joueur.vx * 0.35;
        ballon.vy += ny * force + joueur.vy * 0.20;

        // Le pied donne toujours un coup vers le haut meme sans tir actif
        ballon.vy -= lift;
        let vy_min = if joueur.en_tir { -4.8 } else { -3.0 };
        if ballon.vy > vy_min {
            ballon.vy = vy_min;
        }
    }

    limiter_vitesse_ballon(ballon, 18.0);
}

/// Retourne:
/// - la normale de collision (nx, ny)
/// - la pénétration (distance de chevauchement)
fn collision_rect_cercle(
    rx: f32,
    ry: f32,
    rw: f32,
    rh: f32,
    cx: f32,
    cy: f32,
    cr: f32,
) -> Option<(f32, f32, f32)> {
    let closest_x = cx.clamp(rx, rx + rw);
    let closest_y = cy.clamp(ry, ry + rh);
    let dx = cx - closest_x;
    let dy = cy - closest_y;
    let dist2 = dx * dx + dy * dy;

    if dist2 > cr * cr {
        return None;
    }

    if dist2 > 0.0001 {
        let dist = dist2.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;
        let penetration = cr - dist;
        return Some((nx, ny, penetration));
    }

    // Centre du cercle dans le rectangle: on pousse vers le bord le plus proche.
    let dist_left = (cx - rx).abs();
    let dist_right = (rx + rw - cx).abs();
    let dist_top = (cy - ry).abs();
    let dist_bottom = (ry + rh - cy).abs();

    let min_dist = dist_left.min(dist_right).min(dist_top.min(dist_bottom));

    let (nx, ny) = if min_dist == dist_left {
        (-1.0, 0.0)
    } else if min_dist == dist_right {
        (1.0, 0.0)
    } else if min_dist == dist_top {
        (0.0, -1.0)
    } else {
        (0.0, 1.0)
    };

    let penetration = cr + min_dist;

    Some((nx, ny, penetration))
}

fn limiter_vitesse_ballon(ballon: &mut Ballon, vmax: f32) {
    let speed2 = ballon.vx * ballon.vx + ballon.vy * ballon.vy;
    let vmax2 = vmax * vmax;

    if speed2 > vmax2 {
        let speed = speed2.sqrt();
        let scale = vmax / speed;
        ballon.vx *= scale;
        ballon.vy *= scale;
    }
}