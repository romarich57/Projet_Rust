use crate::models::ball::Ball;
use macroquad::prelude::screen_width;
use macroquad::time::get_frame_time;
use crate::physics::{
    BALL_GRAVITY_REFERENCE, CROSSBAR_THICKNESS_REFERENCE, CROSSBAR_Y_REFERENCE,
    GOAL_MARGIN_REFERENCE, ground_level, scale_x, scale_y,
};

pub fn apply_ball_physics(ball: &mut Ball) {
    let ground_y = ground_level();
    let left_goal_x = GOAL_MARGIN_REFERENCE * scale_x();
    let right_goal_x = screen_width() - GOAL_MARGIN_REFERENCE * scale_x();
    let crossbar_y = CROSSBAR_Y_REFERENCE * scale_y();
    let crossbar_thickness = CROSSBAR_THICKNESS_REFERENCE * scale_y();

    // Framerate independence: 75 FPS reference keeps existing tuning
    let dt = get_frame_time().clamp(1.0 / 240.0, 1.0 / 20.0);
    let scale = dt * 75.0;

    // Gravity
    ball.vy += BALL_GRAVITY_REFERENCE * scale;

    // Update position from velocity
    ball.x += ball.vx * scale;
    ball.y += ball.vy * scale;

    ball.angle += ball.vx * 0.05 * scale;

    let (bcx, bcy, bcr) = ball.circle_hitbox();

    if bcx < left_goal_x || bcx > right_goal_x {
        // Bounce on top of crossbar
        if bcy + bcr > crossbar_y && bcy < crossbar_y && ball.vy > 0.0 {
            ball.y = crossbar_y - bcr - ball.hitbox.offset_y;
            ball.vy = -ball.vy * 0.8;
            ball.vx *= 0.98_f32.powf(scale);
        }
        // Bounce under crossbar
        else if bcy - bcr < crossbar_y + crossbar_thickness && bcy > crossbar_y && ball.vy < 0.0 {
            ball.y = crossbar_y + crossbar_thickness + bcr - ball.hitbox.offset_y;
            ball.vy = -ball.vy * 0.8;
        }
    }

    // If the ball is behind the goal line under the crossbar, damp horizontal speed.
    if (bcx < left_goal_x || bcx > right_goal_x) && bcy > crossbar_y {
        ball.vx *= 0.93_f32.powf(scale);
    }

    // Ground bounce only for meaningful downward impacts.
    if bcy + bcr > ground_y {
        ball.y = ground_y - bcr - ball.hitbox.offset_y;

        if ball.vy > 0.0 {
            let impact_speed = ball.vy;
            let bounce_threshold = 1.1 * scale_y();

            if impact_speed > bounce_threshold {
                ball.vy = -impact_speed * 0.62;
            } else {
                ball.vy = 0.0;
            }
        } else {
            ball.vy = 0.0;
        }

        // Ground friction
        ball.vx *= 0.94_f32.powf(scale);
        if ball.vx.abs() < 0.03 * scale_x() {
            ball.vx = 0.0;
        }
    }

    // Soft force to keep the ball inside the field bounds.
    let return_zone = 120.0 * scale_x();
    let return_force = 0.18 * scale_x() * scale;

    // Left boundary
    if bcx - bcr < 0.0 {
        ball.x = bcr - ball.hitbox.offset_x;
        if ball.vx < 0.0 {
            ball.vx = 0.0;
        }
    }
    let left_distance = (bcx - bcr).max(0.0);
    if left_distance < return_zone {
        let intensity = 1.0 - left_distance / return_zone;
        ball.vx += return_force * intensity;
    }

    // Right boundary
    if bcx + bcr > screen_width() {
        ball.x = screen_width() - bcr - ball.hitbox.offset_x;
        if ball.vx > 0.0 {
            ball.vx = 0.0;
        }
    }
    let right_distance = (screen_width() - (bcx + bcr)).max(0.0);
    if right_distance < return_zone {
        let intensity = 1.0 - right_distance / return_zone;
        ball.vx -= return_force * intensity;
    }

    // Ceiling collision
    if bcy - bcr < 0.0 {
        ball.y = bcr + ball.hitbox.offset_y;
        ball.vy = -ball.vy * 0.6;
    }
}