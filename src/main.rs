mod models;
mod render;
mod physics;
mod input;

use macroquad::prelude::*;
use models::ballon::Ballon;
use models::joueur::Joueur;
use physics::player_physics;

const HITBOX_PIED_LARGEUR_COEF: f32 = 1.0;
const HITBOX_PIED_HAUTEUR_COEF: f32 = 1.0;
const HITBOX_TETE_LARGEUR_COEF: f32 = 1.0;
const HITBOX_TETE_HAUTEUR_COEF: f32 = 1.0;

fn appliquer_tuning_hitbox_joueur(joueur: &mut Joueur) {
    joueur.set_hitbox_pied(
        joueur.hitbox_pied.offset_x,
        joueur.hitbox_pied.offset_y,
        joueur.hitbox_pied.largeur * HITBOX_PIED_LARGEUR_COEF,
        joueur.hitbox_pied.hauteur * HITBOX_PIED_HAUTEUR_COEF,
    );

    joueur.set_hitbox_tete(
        joueur.hitbox_tete.offset_x,
        joueur.hitbox_tete.offset_y,
        joueur.hitbox_tete.largeur * HITBOX_TETE_LARGEUR_COEF,
        joueur.hitbox_tete.hauteur * HITBOX_TETE_HAUTEUR_COEF,
    );
}

fn configuration_fenetre() -> Conf {
    Conf {
        window_title: "Head Soccer".to_owned(),
        window_width: 1000,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(configuration_fenetre())]
async fn main() {
    let t_ballon = load_texture("src/assets/ballon/ballon.png").await.unwrap();
    let mut ballon = Ballon::new(
        screen_width() / 2.0,
        physics::niveau_sol() + 200.0 * physics::echelle_y(),
        30.0 * physics::echelle_x().min(physics::echelle_y()),
        t_ballon,
    );
    ballon.set_hitbox_cercle(0.0, 0.0, ballon.rayon_visuel() * 0.7);

    let t_stade = load_texture("src/assets/stade/stade.png").await.unwrap();
    let t_tete = load_texture("src/assets/joueur/tete.png").await.unwrap();
    let t_pied = load_texture("src/assets/joueur/pied.png").await.unwrap();

    let mut joueur = Joueur::new(0.0, 0.0, t_tete, t_pied);
    joueur.appliquer_taille_relative_ecran(screen_width(), screen_height());
    appliquer_tuning_hitbox_joueur(&mut joueur);
    joueur.y = joueur.y_pose_au_sol(physics::niveau_sol());
    joueur.x = screen_width() - joueur.largeur_collision() - 20.0 * physics::echelle_x();

    let mut derniere_largeur = screen_width();
    let mut derniere_hauteur = screen_height();
    let mut debug_hitbox = false;

    loop {
        if (screen_width() - derniere_largeur).abs() > f32::EPSILON
            || (screen_height() - derniere_hauteur).abs() > f32::EPSILON
        {
            joueur.appliquer_taille_relative_ecran(screen_width(), screen_height());
            appliquer_tuning_hitbox_joueur(&mut joueur);
            joueur.y = joueur.y_pose_au_sol(physics::niveau_sol());

            let marge_droite = 20.0 * physics::echelle_x();
            let x_max = screen_width() - joueur.largeur_collision() - marge_droite;
            joueur.x = joueur.x.clamp(0.0, x_max);

            derniere_largeur = screen_width();
            derniere_hauteur = screen_height();
        }

        if is_key_pressed(KeyCode::Y) {
            debug_hitbox = !debug_hitbox;
        }

        input::gerer_clavier(&mut joueur);
        input::update_animations(&mut joueur);

        player_physics::appliquer_physique(&mut joueur);

        physics::collision::appliquer_collision_joueur_ballon(&joueur, &mut ballon);

        physics::ball_physics::appliquer_physique_ballon(&mut ballon);

        render::dessiner_tout(&joueur, &t_stade, &ballon, debug_hitbox);

        next_frame().await
    }
}