mod app;
mod arcade_ui;
mod game;
mod gameplay;
mod ia;
mod input;
mod match_arena;
mod match_hud;
#[path = "choix_joueur_temps/mod.rs"]
mod match_setup;
mod menu;
#[path = "choix_mode/mod.rs"]
mod mode_selection;
#[path = "Leaderboard/mod.rs"]
mod leaderboard;
mod models;
mod physics;
mod render;

use app::App;
use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Head Soccer".to_owned(),
        window_width: 1000,
        window_height: 600,
        window_resizable: true,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config())]
async fn main() {
    let mut app = App::new()
        .await
        .unwrap_or_else(|err| panic!("failed to initialize application: {err}"));

    loop {
        app.update();
        app.draw();
        next_frame().await;
    }
}
