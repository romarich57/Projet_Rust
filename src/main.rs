mod models;
mod render;
mod physics;
mod input;
mod ia;

use macroquad::prelude::*;
use models::ball::Ball;
use models::player::Player;
use physics::player_physics;
use models::player::ControlType;

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

    let mut players = vec![
        Player::new(
            0.0,
            0.0,
            head_texture.clone(), // Clone to avoid move semantics issues because Texture2D is not Copy
            foot_texture.clone(),
            ControlType::Player1,
            1,
        ),
        Player::new(
            0.0,
            0.0,
            head_texture,
            foot_texture,
            ControlType::IA,
            -1,
        ),
    ];

    for player in &mut players {
        player.apply_relative_screen_size(screen_width(), screen_height());
        apply_player_hitbox_tuning(player);
        player.y = player.y_at_ground(physics::ground_level());

        let margin = 20.0 * physics::scale_x();
        player.x = if player.side < 0 {
            margin
        } else {
            screen_width() - player.collision_width() - margin
        };
    }

    let mut last_width = screen_width();
    let mut last_height = screen_height();
    let mut debug_hitbox = false;

    loop {
        if (screen_width() - last_width).abs() > f32::EPSILON
            || (screen_height() - last_height).abs() > f32::EPSILON
        {
            for player in &mut players {
                player.apply_relative_screen_size(screen_width(), screen_height());
                apply_player_hitbox_tuning(player);
                player.y = player.y_at_ground(physics::ground_level());

                let right_margin = 20.0 * physics::scale_x();
                let x_max = screen_width() - player.collision_width() - right_margin;
                player.x = player.x.clamp(0.0, x_max);
            }

            last_width = screen_width();
            last_height = screen_height();
        }

        if is_key_pressed(KeyCode::Y) {
            debug_hitbox = !debug_hitbox;
        }

        for player in &mut players {
            match player.control_type {
                ControlType::IA => ia::handle_ai(player, &ball),
                _ => input::handle_keyboard(player),
            }
            input::update_animations(player);
            player_physics::apply_physics(player);
        }

        if players.len() >= 2 {
            let (left, right) = players.split_at_mut(1);
            physics::collision::apply_player_player_collision(&mut left[0], &mut right[0]);
        }

        for player in &players {
            physics::collision::apply_player_ball_collision(player, &mut ball);
        }

        physics::ball_physics::apply_ball_physics(&mut ball);

        render::draw_all(&players, &stadium_texture, &ball, debug_hitbox);

        next_frame().await
    }
}