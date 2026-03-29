use crate::match_arena::ArenaGeometry;
use crate::models::ball::Ball;
use crate::models::player::Player;
use macroquad::prelude::screen_width;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub fn handle_ai(player: &mut Player, ball: &Ball, arena: &ArenaGeometry, difficulty: Difficulty) {
    let field_w = screen_width();
    let field_mid = field_w * 0.5;
    let is_left_side = player.side < 0;

    let own_goal_x = if is_left_side {
        arena.left_goal.mouth_line_x
    } else {
        arena.right_goal.mouth_line_x
    };

    let attack_dir = if is_left_side { 1.0 } else { -1.0 }; // Direction towards the opponent's goal (positive for left side, negative for right side)
    let player_center_x = player.x + player.collision_width() * 0.5;
    let player_head_y = player.y + player.head_offset_y;
    let player_foot_y = player.y + player.foot_height * 0.5;

    /*
        prediction_time : How far into the future the AI should predict the ball's position.
        move_speed_normal : The horizontal movement speed of the AI when the ball is not deemed dangerous.
        move_speed_danger : The horizontal movement speed of the AI when the ball is deemed dangerous (heading towards the player's own goal).
        x_dead_zone : A threshold distance in the x-axis within which the AI will not attempt to move, to prevent jittery movement when the target position is very close.
        jump_reach_factor : A multiplier that determines how far in the x-axis the AI considers the ball to be within reach for jumping.
        jump_normal : The vertical velocity applied to the AI when it decides to jump under normal circumstances.
        jump_danger : The vertical velocity applied to the AI when it decides to jump in a dangerous situation (when the ball is heading towards the player's own goal).
        shot_x_factor : A multiplier that determines how close the ball needs to be in the x-axis for the AI to consider shooting.
        shot_y_factor : A multiplier that determines how close the ball needs to be in the y-axis for the AI to consider shooting.
        front_tolerance : A threshold that determines how far in front of the player the ball needs to be for the AI to consider it a valid shooting opportunity, to prevent the AI from trying to shoot when the ball is behind them.
        home_left_ratio : A ratio that determines the x-coordinate of the AI's "home" position on the field, which is a default position the AI will return to when the ball is not in a threatening position.
        engage_range : A ratio that determines how close the ball needs to be to the player for the AI to switch from a more defensive positioning to a more aggressive, attacking positioning.
        own_half_offset : A multiplier that determines how far from the predicted ball position the AI should position itself when the ball is in its own half but not deemed dangerous, to allow the AI to better intercept passes.
        attack_offset : A multiplier that determines how far from the predicted ball position the AI should position itself when the ball is in the opponent's half, to allow the AI to better engage in attacking plays.
    */
    let (
        prediction_time,
        move_speed_normal,
        move_speed_danger,
        x_dead_zone,
        jump_reach_factor,
        jump_normal,
        jump_danger,
        shot_x_factor,
        shot_y_factor,
        front_tolerance,
        home_left_ratio,
        engage_range,
        own_half_offset,
        attack_offset,
    ) =
        match difficulty {
            Difficulty::Easy => (7.5, 2.35, 2.5, 8.0, 0.48, -8.3, -9.0, 0.52, 0.80, -5.0, 0.45, 0.45, 0.20, 0.18),
            Difficulty::Normal => (10.0, 2.6, 3.0, 11.0, 0.52, -9.0, -10.0, 0.70, 0.85, -6.5, 0.40, 0.35, 0.28, 0.24),
            Difficulty::Hard => (13.5, 3.15, 3.7, 6.5, 0.62, -10.2, -11.3, 0.62, 0.98, -10.0, 0.39, 0.34, 0.25, 0.22),
        };

    let predicted_ball_x = (ball.x + ball.vx * prediction_time).clamp(0.0, field_w); //clamp is used to ensure the predicted position doesn't go beyond the field boundaries
    let predicted_ball_y = ball.y + ball.vy * prediction_time;

    let ball_toward_own_goal = (ball.vx * attack_dir) < -0.2; // Is the ball moving towards the player's own goal?
    let ball_in_own_half = if is_left_side { // For left side players, the ball is in their half if it's on the left side of the field.
        predicted_ball_x < field_mid
    } else {
        predicted_ball_x > field_mid
    };
    let dangerous_ball = ball_toward_own_goal && ball_in_own_half; // Is the ball a threat to the player's own goal?

    let home_x = if is_left_side {
        field_w * home_left_ratio
    } else {
        field_w * (1.0 - home_left_ratio)
    };

    let defend_x = (own_goal_x * 0.40 + predicted_ball_x * 0.60) //The defend position is just after the next predicted ball position to allow the AI to intercept the ball
        .clamp(arena.player_left_wall_x, arena.player_right_wall_x);

    let attack_x = (predicted_ball_x - player.collision_width() * attack_offset).clamp(
        arena.player_left_wall_x,
        arena.player_right_wall_x - player.collision_width(),
    );

    let target_x = if dangerous_ball {
        defend_x
    } else if ball_in_own_half {
        predicted_ball_x - player.collision_width() * own_half_offset
    } else if (ball.x - player_center_x).abs() < field_w * engage_range {
        attack_x
    } else {
        home_x
    };

    let dx = target_x - player.x;
    let move_speed = if dangerous_ball {
        move_speed_danger
    } else {
        move_speed_normal
    };
    if dx.abs() > x_dead_zone {
        player.vx = dx.signum() * move_speed;
    } else {
        player.vx = 0.0;
    }
    //Jumping Logic:
    //The AI will verify if it's on the ground and has jumps available, then check if the ball is reachable in the x-axis 
    // and if it's descending towards the player. If the ball is deemed dangerous (heading towards the player's own goal), 
    // the AI will be more aggressive in jumping to intercept it, even if it's slightly above the player's head.
    let on_ground = player.y >= player.y_at_ground(arena.ground_y) - 2.0;
    let ball_reachable_x = (ball.x - player_center_x).abs() < player.collision_width() * jump_reach_factor;
    let ball_descending_on_player = predicted_ball_y > player_head_y - player.head_height * 0.2
        && predicted_ball_y < player_foot_y;
    let lob_intercept_window = (ball.x - player_center_x).abs() < player.collision_width() * (jump_reach_factor + 0.2)
        && ball.vy > 0.35
        && predicted_ball_y < player_head_y - player.head_height * 0.25;

    if on_ground
        && player.jump_count < 2
        && ball_reachable_x
        && (ball_descending_on_player
            || lob_intercept_window
            || (dangerous_ball && ball.y < player_head_y + 25.0))
    {
        player.vy = if dangerous_ball { jump_danger } else { jump_normal };
        player.jump_count += 1;
    }

    // Shooting Logic:
    // The AI will attempt to shoot if the ball is close enough to the player's foot and
    // is in front of the player (to avoid trying to shoot when the ball is behind them).
    let ball_in_front = (ball.x - player_center_x) * attack_dir > front_tolerance;
    let ball_close_for_shot = (ball.x - player_center_x).abs() < player.collision_width() * shot_x_factor
        && (ball.y - player_foot_y).abs() < player.foot_height * shot_y_factor;

    if ball_close_for_shot && ball_in_front {
        player.is_shooting = true;
    }
}
