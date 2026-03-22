use crate::models::ballon::Ballon;
use crate::models::joueur::Joueur;

/// Collision joueur <-> ballon:
/// - collision "corps/tête" (faible impulsion)
/// - collision "pied en tir" (forte impulsion)
pub fn appliquer_collision_joueur_ballon(joueur: &Joueur, ballon: &mut Ballon) {
    let (pied_x, pied_y, pied_l, pied_h) = joueur.hitbox_rect_pied_active();
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

        let progression_tir = (-joueur.angle_pied).clamp(0.0, 1.0);
        let tir_en_phase = joueur.en_tir || progression_tir > 0.22;

        if tir_en_phase {
            // Direction de frappe: plus le pied monte (angle important),
            // plus la balle prend de hauteur.
            let contact_y = ((bcy - pied_y) / pied_h).clamp(0.0, 1.0);
            let bonus_lob = (1.0 - contact_y) * 0.35;

            let dir_x = if nx.abs() > 0.05 { nx } else { -1.0 };
            let dir_y = -(0.35 + 0.70 * progression_tir + bonus_lob);

            let force = 8.5 + 6.5 * progression_tir;
            let transfert_vitesse = joueur.vx * 0.45;

            ballon.vx += dir_x * force + transfert_vitesse;
            ballon.vy += dir_y * force + joueur.vy * 0.10;

            // Assure une montée minimale sur une vraie frappe.
            let vy_min = -(3.8 + 2.6 * progression_tir);
            if ballon.vy > vy_min {
                ballon.vy = vy_min;
            }
        } else {
            // Contact doux hors tir: on réduit la vitesse générale et on
            // applique juste une petite impulsion naturelle.
            let force_douce = 1.6;
            ballon.vx *= 0.86;
            ballon.vy *= 0.90;

            ballon.vx += nx * force_douce + joueur.vx * 0.20;
            ballon.vy += ny * (force_douce * 0.55);
            ballon.vy -= 0.35;
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