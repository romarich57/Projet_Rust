mod models;
mod render;
mod physics;
mod input;
mod game;

use macroquad::prelude::*;
use models::ball::Ball;
use models::player::Player;
use physics::player_physics;
use game::Match;

const FOOT_HITBOX_WIDTH_COEF: f32 = 1.0;
const FOOT_HITBOX_HEIGHT_COEF: f32 = 1.0;
const HEAD_HITBOX_WIDTH_COEF: f32 = 1.0;
const HEAD_HITBOX_HEIGHT_COEF: f32 = 1.0;

fn apply_player_hitbox_tuning(player: &mut Player) {
    player.set_foot_hitbox(
        player.foot_hitbox.offset_x,
        player.foot_hitbox.offset_y,
        player.foot_hitbox.width * FOOT_HITBOX_WIDTH_COEF,
        player.foot_hitbox.height * FOOT_HITBOX_HEIGHT_COEF,
    );

    player.set_head_hitbox(
        player.head_hitbox.offset_x,
        player.head_hitbox.offset_y,
        player.head_hitbox.width * HEAD_HITBOX_WIDTH_COEF,
        player.head_hitbox.height * HEAD_HITBOX_HEIGHT_COEF,
    );
}

fn window_config() -> Conf {
    Conf {
        window_title: "Head Soccer".to_owned(),
        window_width: 1000,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config())]
async fn main() {
    let ball_texture = load_texture("src/assets/ballon/ballon.png").await.unwrap();
    let mut ball = Ball::new(
        screen_width() / 2.0,
        physics::ground_level() + 200.0 * physics::scale_y(),
        30.0 * physics::scale_x().min(physics::scale_y()),
        ball_texture,
    );
    ball.set_circle_hitbox(0.0, 0.0, ball.visual_radius() * 0.7);

    let stadium_texture = load_texture("src/assets/stade/stade.png").await.unwrap();
    let head_texture = load_texture("src/assets/joueur/tete.png").await.unwrap();
    let foot_texture = load_texture("src/assets/joueur/pied.png").await.unwrap();

    let mut player = Player::new(0.0, 0.0, head_texture, foot_texture);
    player.apply_relative_screen_size(screen_width(), screen_height());
    apply_player_hitbox_tuning(&mut player);
    player.y = player.y_at_ground(physics::ground_level());
    player.x = screen_width() - player.collision_width() - 20.0 * physics::scale_x();

    let start_x_player = screen_width() - player.collision_width() - 20.0 * physics::scale_x();
    player.x = start_x_player;

    let mut last_width = screen_width();
    let mut last_height = screen_height();
    let mut debug_hitbox = false;

    let mut soccer_match = Match::new();

    loop {
        if (screen_width() - last_width).abs() > f32::EPSILON
            || (screen_height() - last_height).abs() > f32::EPSILON
        {
            player.apply_relative_screen_size(screen_width(), screen_height());
            apply_player_hitbox_tuning(&mut player);
            player.y = player.y_at_ground(physics::ground_level());

            let right_margin = 20.0 * physics::scale_x();
            let x_max = screen_width() - player.collision_width() - right_margin;
            player.x = player.x.clamp(0.0, x_max);

            last_width = screen_width();
            last_height = screen_height();
        }

        if is_key_pressed(KeyCode::Y) {
            debug_hitbox = !debug_hitbox;
        }

        input::handle_keyboard(&mut player);
        input::update_animations(&mut player);

        player_physics::apply_physics(&mut player);

        physics::collision::apply_player_ball_collision(&player, &mut ball);

        physics::ball_physics::apply_ball_physics(&mut ball);

        soccer_match.update(&mut ball, &mut player, start_x_player);

        render::draw_all(&player, &stadium_texture, &ball, debug_hitbox, &soccer_match);

        next_frame().await
    }
}