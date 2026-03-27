use macroquad::prelude::*;
use crate::models::ball::Ball;
use crate::models::player::Player;

pub fn draw_all(player: &Player, stadium_texture: &Texture2D, ball: &Ball, debug_hitbox: bool) {
    draw_texture_ex(stadium_texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    });

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

    draw_texture_ex(
        &player.foot_texture,
        player.x,
        player.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(player.foot_width, player.foot_height)),
            rotation: player.foot_angle,
            ..Default::default()
        },
    );

    draw_texture_ex(
        &player.head_texture,
        player.x + player.head_offset_x,
        player.y + player.head_offset_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(player.head_width, player.head_height)),
            ..Default::default()
        },
    );

    if debug_hitbox {
        draw_debug_hitbox(player, ball);
    }
}

fn draw_debug_hitbox(player: &Player, ball: &Ball) {
    let (foot_base_x, foot_base_y, foot_base_w, foot_base_h) = player.foot_hitbox_rect();
    let (foot_x, foot_y, foot_w, foot_h) = player.active_foot_hitbox_rect();
    let (head_x, head_y, head_w, head_h) = player.head_hitbox_rect();

    let ball_hitbox = ball.circle_hitbox();
    let ball_cx = ball_hitbox.0;
    let ball_cy = ball_hitbox.1;
    let ball_r = ball_hitbox.2;

    draw_rectangle_lines(foot_base_x, foot_base_y, foot_base_w, foot_base_h, 1.0, PINK);
    draw_rectangle_lines(foot_x, foot_y, foot_w, foot_h, 2.0, RED);
    draw_rectangle_lines(head_x, head_y, head_w, head_h, 2.0, ORANGE);
    draw_rectangle_lines(
        ball_cx - ball_r,
        ball_cy - ball_r,
        ball_r * 2.0,
        ball_r * 2.0,
        2.0,
        LIME,
    );

    draw_text("DEBUG HITBOX (Y)", 15.0, 30.0, 26.0, YELLOW);
}