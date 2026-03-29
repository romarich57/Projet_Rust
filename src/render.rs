use crate::arcade_ui::draw_cover_texture;
use crate::game::{GameState, Match};
use crate::match_arena::ArenaGeometry;
use crate::models::ball::Ball;
use crate::models::player::Player;
use macroquad::prelude::*;

pub fn draw_all(
    players: &[Player],
    terrain_texture: &Texture2D,
    goal_texture: &Texture2D,
    arena: &ArenaGeometry,
    ball: &Ball,
    debug_hitbox: bool,
    game_match: &Match,
) {
    draw_cover_texture(
        terrain_texture,
        Rect::new(0.0, 0.0, arena.screen_width, arena.screen_height),
    );

    draw_goal_texture(goal_texture, arena.left_goal.draw_rect, false);
    draw_goal_texture(goal_texture, arena.right_goal.draw_rect, true);

    draw_texture_ex(
        &ball.texture,
        ball.x - ball.visual_radius(),
        ball.y - ball.visual_radius(),
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(ball.visual_radius() * 2.0, ball.visual_radius() * 2.0)),
            rotation: ball.angle,
            ..Default::default()
        },
    );

    for player in players {
        draw_player(player);
    }

    draw_state_overlay(game_match);

    if debug_hitbox {
        draw_goal_debug(arena.left_goal);
        draw_goal_debug(arena.right_goal);
        for player in players {
            draw_debug_hitbox(player, ball);
        }
        draw_text("DEBUG HITBOX (Y)", 15.0, 30.0, 26.0, YELLOW);
    }
}

fn draw_goal_texture(texture: &Texture2D, rect: Rect, flip_x: bool) {
    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(rect.w, rect.h)),
            flip_x,
            ..Default::default()
        },
    );
}

fn draw_player(player: &Player) {
    let foot_draw_x = player.x;
    let foot_pivot = player.foot_pivot_screen_pos();
    draw_texture_ex(
        &player.foot_texture,
        foot_draw_x,
        player.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(player.foot_width, player.foot_height)),
            rotation: player.foot_angle,
            flip_x: player.faces_right(),
            pivot: Some(foot_pivot),
            ..Default::default()
        },
    );

    let head_draw_x = player.x + player.head_offset_x;
    draw_texture_ex(
        &player.head_texture,
        head_draw_x,
        player.y + player.head_offset_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(player.head_width, player.head_height)),
            ..Default::default()
        },
    );
}

fn draw_state_overlay(game_match: &Match) {
    match game_match.state {
        GameState::GoalScored { .. } => {
            let goal_text = "GOAL !";
            let goal_size = 140.0;
            let goal_params = measure_text(goal_text, None, goal_size as u16, 1.0);

            let text_x = screen_width() / 2.0 - goal_params.width / 2.0;
            let text_y = screen_height() / 2.0 + goal_params.height / 2.0 - 30.0;

            draw_text(goal_text, text_x + 8.0, text_y + 8.0, goal_size, BLACK);
            draw_text(goal_text, text_x, text_y, goal_size, YELLOW);
        }
        GameState::Finished => {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.42),
            );

            let title = "FIN DU MATCH";
            let subtitle = format!(
                "Score final: {} - {}",
                game_match.score_p1, game_match.score_p2
            );
            let title_size = 84.0;
            let subtitle_size = 34.0;

            let title_metrics = measure_text(title, None, title_size as u16, 1.0);
            let subtitle_metrics = measure_text(&subtitle, None, subtitle_size as u16, 1.0);

            let title_x = screen_width() * 0.5 - title_metrics.width * 0.5;
            let title_y = screen_height() * 0.5 - 10.0;
            let subtitle_x = screen_width() * 0.5 - subtitle_metrics.width * 0.5;
            let subtitle_y = title_y + 54.0;

            draw_text(title, title_x + 5.0, title_y + 5.0, title_size, BLACK);
            draw_text(
                title,
                title_x,
                title_y,
                title_size,
                color_u8!(255, 236, 132, 255),
            );
            draw_text(
                &subtitle,
                subtitle_x + 2.0,
                subtitle_y + 2.0,
                subtitle_size,
                BLACK,
            );
            draw_text(&subtitle, subtitle_x, subtitle_y, subtitle_size, WHITE);
        }
        GameState::Playing => {}
    }
}

fn draw_debug_hitbox(player: &Player, ball: &Ball) {
    let (foot_base_x, foot_base_y, foot_base_w, foot_base_h) = player.foot_hitbox_rect();
    let (foot_x, foot_y, foot_w, foot_h) = player.active_foot_hitbox_rect();
    let (head_x, head_y, head_w, head_h) = player.head_hitbox_rect();
    let (body_x, body_y, body_w, body_h) = player.body_hitbox_rect();

    let ball_hitbox = ball.circle_hitbox();
    let ball_cx = ball_hitbox.0;
    let ball_cy = ball_hitbox.1;
    let ball_r = ball_hitbox.2;

    draw_rectangle_lines(
        foot_base_x,
        foot_base_y,
        foot_base_w,
        foot_base_h,
        1.0,
        PINK,
    );
    draw_rectangle_lines(foot_x, foot_y, foot_w, foot_h, 2.0, RED);
    draw_rectangle_lines(head_x, head_y, head_w, head_h, 2.0, ORANGE);
    if body_h > 0.0 {
        draw_rectangle_lines(body_x, body_y, body_w, body_h, 2.0, BLUE);
    }
    draw_rectangle_lines(
        ball_cx - ball_r,
        ball_cy - ball_r,
        ball_r * 2.0,
        ball_r * 2.0,
        2.0,
        LIME,
    );
}

fn draw_goal_debug(goal: crate::match_arena::GoalGeometry) {
    for rect in [
        goal.field_post_tip_rect,
        goal.back_post_rect,
        goal.crossbar_rect,
        goal.goal_floor_rect,
        goal.goal_cavity_rect,
        goal.goal_capture_rect,
    ] {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, SKYBLUE);
    }

    draw_line(
        goal.mouth_line_x,
        goal.opening_top_y,
        goal.mouth_line_x,
        goal.opening_bottom_y,
        2.0,
        YELLOW,
    );
}
