use crate::match_arena::ArenaGeometry;
use crate::models::ball::Ball;
use crate::models::player::Player;
use macroquad::prelude::screen_width;

pub fn handle_ai(player: &mut Player, ball: &Ball, arena: &ArenaGeometry) {
    let field_w = screen_width();
    let field_mid = field_w * 0.5;
    let is_left_side = player.side < 0;

    let own_goal_x = if is_left_side {
        arena.left_goal.goal_line_x
    } else {
        arena.right_goal.goal_line_x
    };

    let attack_dir = if is_left_side { 1.0 } else { -1.0 };
    let player_center_x = player.x + player.collision_width() * 0.5;
    let player_head_y = player.y + player.head_offset_y;
    let player_foot_y = player.y + player.foot_height * 0.5;

    let prediction_time = 12.0;
    let predicted_ball_x = (ball.x + ball.vx * prediction_time).clamp(0.0, field_w);
    let predicted_ball_y = ball.y + ball.vy * prediction_time;

    let ball_toward_own_goal = (ball.vx * attack_dir) < -0.2;
    let ball_in_own_half = if is_left_side {
        predicted_ball_x < field_mid
    } else {
        predicted_ball_x > field_mid
    };
    let dangerous_ball = ball_toward_own_goal && ball_in_own_half;

    let home_x = if is_left_side {
        field_w * 0.32
    } else {
        field_w * 0.68
    };

    let defend_x = (own_goal_x * 0.40 + predicted_ball_x * 0.60)
        .clamp(arena.player_left_wall_x, arena.player_right_wall_x);

    let attack_x = (predicted_ball_x - player.collision_width() * 0.45).clamp(
        arena.player_left_wall_x,
        arena.player_right_wall_x - player.collision_width(),
    );

    let target_x = if dangerous_ball {
        defend_x
    } else if ball_in_own_half {
        predicted_ball_x - player.collision_width() * 0.35
    } else if (ball.x - player_center_x).abs() < field_w * 0.22 {
        attack_x
    } else {
        home_x
    };

    let dx = target_x - player.x;
    let move_speed = if dangerous_ball { 3.3 } else { 2.9 };
    if dx.abs() > 8.0 {
        player.vx = dx.signum() * move_speed;
    } else {
        player.vx = 0.0;
    }

    let on_ground = player.y >= player.y_at_ground(arena.ground_y) - 2.0;
    let ball_reachable_x = (ball.x - player_center_x).abs() < player.collision_width() * 0.55;
    let ball_descending_on_player = predicted_ball_y > player_head_y - player.head_height * 0.2
        && predicted_ball_y < player_foot_y;

    if on_ground
        && player.jump_count < 2
        && ball_reachable_x
        && (ball_descending_on_player || (dangerous_ball && ball.y < player_head_y + 25.0))
    {
        player.vy = if dangerous_ball { -10.5 } else { -9.5 };
        player.jump_count += 1;
    }

    let ball_in_front = (ball.x - player_center_x) * attack_dir > -8.0;
    let ball_close_for_shot = (ball.x - player_center_x).abs() < player.collision_width() * 0.5
        && (ball.y - player_foot_y).abs() < player.foot_height * 0.9;

    if ball_close_for_shot && ball_in_front {
        player.is_shooting = true;
    }
}
