use crate::match_arena::{ArenaGeometry, GoalGeometry};
use crate::models::ball::Ball;
use crate::physics::{scale_x, scale_y, BALL_GRAVITY_REFERENCE};
use macroquad::prelude::*;
use macroquad::time::get_frame_time;

pub fn apply_ball_physics(ball: &mut Ball, arena: &ArenaGeometry) {
    let dt = get_frame_time().clamp(1.0 / 240.0, 1.0 / 20.0);
    let scale = dt * 75.0;

    ball.vy += BALL_GRAVITY_REFERENCE * scale;
    ball.x += ball.vx * scale;
    ball.y += ball.vy * scale;
    ball.angle += ball.vx * 0.05 * scale;

    resolve_goal_collisions(ball, arena.left_goal);
    resolve_goal_collisions(ball, arena.right_goal);

    if arena.ball_in_goal_net(ball).is_some() {
        ball.vx *= 0.985_f32.powf(scale);
        ball.vy *= 0.992_f32.powf(scale);
    }

    let (center_x, center_y, radius) = ball.circle_hitbox();

    if center_y + radius > arena.ground_y {
        ball.y = arena.ground_y - radius - ball.hitbox.offset_y;

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

        ball.vx *= 0.94_f32.powf(scale);
        if ball.vx.abs() < 0.03 * scale_x() {
            ball.vx = 0.0;
        }
    }

    if center_x - radius < 0.0 {
        ball.x = radius - ball.hitbox.offset_x;
        if ball.vx < 0.0 {
            ball.vx = -ball.vx * 0.35;
        }
    }

    if center_x + radius > arena.screen_width {
        ball.x = arena.screen_width - radius - ball.hitbox.offset_x;
        if ball.vx > 0.0 {
            ball.vx = -ball.vx * 0.35;
        }
    }

    if center_y - radius < arena.hud_height {
        ball.y = arena.hud_height + radius - ball.hitbox.offset_y;
        if ball.vy < 0.0 {
            ball.vy = -ball.vy * 0.6;
        }
    }
}

fn resolve_goal_collisions(ball: &mut Ball, goal: GoalGeometry) {
    for rect in [
        goal.front_post_rect,
        goal.back_post_rect,
        goal.crossbar_rect,
    ] {
        if let Some((nx, ny, penetration)) = circle_rect_collision(ball, rect) {
            ball.x += nx * penetration;
            ball.y += ny * penetration;

            let incoming = vec2(ball.vx, ball.vy);
            let normal = vec2(nx, ny);
            let reflected = incoming - 2.0 * incoming.dot(normal) * normal;

            ball.vx = reflected.x * 0.78;
            ball.vy = reflected.y * 0.78;

            if ny < -0.1 {
                ball.vx *= 0.98;
            }
        }
    }
}

fn circle_rect_collision(ball: &Ball, rect: Rect) -> Option<(f32, f32, f32)> {
    let (center_x, center_y, radius) = ball.circle_hitbox();
    let closest_x = center_x.clamp(rect.x, rect.right());
    let closest_y = center_y.clamp(rect.y, rect.bottom());
    let dx = center_x - closest_x;
    let dy = center_y - closest_y;
    let distance_sq = dx * dx + dy * dy;

    if distance_sq > radius * radius {
        return None;
    }

    if distance_sq > 0.0001 {
        let distance = distance_sq.sqrt();
        return Some((dx / distance, dy / distance, radius - distance));
    }

    let left = (center_x - rect.x).abs();
    let right = (rect.right() - center_x).abs();
    let top = (center_y - rect.y).abs();
    let bottom = (rect.bottom() - center_y).abs();
    let min_distance = left.min(right).min(top.min(bottom));

    let (nx, ny) = if min_distance == left {
        (-1.0, 0.0)
    } else if min_distance == right {
        (1.0, 0.0)
    } else if min_distance == top {
        (0.0, -1.0)
    } else {
        (0.0, 1.0)
    };

    Some((nx, ny, radius + min_distance))
}
