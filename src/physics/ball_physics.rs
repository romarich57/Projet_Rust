use crate::match_arena::{ArenaGeometry, GoalGeometry};
use crate::models::ball::Ball;
use crate::physics::{scale_x, scale_y, BALL_GRAVITY_REFERENCE};
use macroquad::prelude::*;
use macroquad::time::get_frame_time;

const CROSSBAR_MIN_REBOUND_SPEED: f32 = 2.0;
const CROSSBAR_TOP_EPSILON: f32 = 0.35;
const CROSSBAR_STATIC_ESCAPE_VX: f32 = 1.5;

pub fn apply_ball_physics(ball: &mut Ball, arena: &ArenaGeometry) {
    let dt = get_frame_time().clamp(1.0 / 240.0, 1.0 / 20.0);
    let scale = dt * 75.0;

    ball.vy += BALL_GRAVITY_REFERENCE * scale;
    ball.x += ball.vx * scale;
    ball.y += ball.vy * scale;
    ball.angle += ball.vx * 0.05 * scale;

    resolve_goal_collisions(ball, arena.left_goal);
    resolve_goal_collisions(ball, arena.right_goal);

    if let Some(goal_side) = arena.ball_in_goal_net(ball) {
        let goal = match goal_side {
            crate::match_arena::GoalSide::Left => arena.left_goal,
            crate::match_arena::GoalSide::Right => arena.right_goal,
        };

        enforce_goal_retention(ball, goal);
        ball.vx *= 0.975_f32.powf(scale);
        ball.vy *= 0.988_f32.powf(scale);
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
    // On active le rebond de sécurité pour TOUT le toit de la cage (transversale ET poteau arrière)
    resolve_goal_rect_collision(ball, goal.crossbar_rect, true, goal.side);
    resolve_goal_rect_collision(ball, goal.back_post_rect, true, goal.side); // Passé à true !

    // Le bout du poteau et le sol restent normaux
    resolve_goal_rect_collision(ball, goal.field_post_tip_rect, false, goal.side);
    resolve_goal_rect_collision(ball, goal.goal_floor_rect, false, goal.side);
}
fn enforce_goal_retention(ball: &mut Ball, goal: GoalGeometry) {
    let (center_x, center_y, radius) = ball.circle_hitbox();
    let center = vec2(center_x, center_y);

    if !rect_circle_overlap(goal.goal_cavity_rect, center, radius) {
        return;
    }

    if let Some((retained_center_x, retained_vx)) =
        retained_ball_state(center_x, radius, ball.vx, goal)
    {
        ball.x = retained_center_x - ball.hitbox.offset_x;
        ball.vx = retained_vx;
    }

    let (_, corrected_center_y, _) = ball.circle_hitbox();
    if corrected_center_y > goal.goal_floor_rect.y && ball.vy > 0.0 {
        ball.vy = -ball.vy.abs() * 0.12;
    }
}

fn retained_ball_state(
    center_x: f32,
    radius: f32,
    vx: f32,
    goal: GoalGeometry,
) -> Option<(f32, f32)> {
    match goal.side {
        crate::match_arena::GoalSide::Left if center_x + radius > goal.mouth_line_x && vx > 0.0 => {
            Some((goal.mouth_line_x - radius - 0.5, -vx.abs() * 0.18))
        }
        crate::match_arena::GoalSide::Right
            if center_x - radius < goal.mouth_line_x && vx < 0.0 =>
        {
            Some((goal.mouth_line_x + radius + 0.5, vx.abs() * 0.18))
        }
        _ => None,
    }
}

fn resolve_goal_rect_collision(
    ball: &mut Ball,
    rect: Rect,
    is_top_bounce_active: bool,
    side: crate::match_arena::GoalSide,
) {
    if let Some((normal, penetration)) = circle_rect_collision(ball, rect) {
        ball.x += normal.x * penetration;
        ball.y += normal.y * penetration;

        let reflected = reflect_velocity(vec2(ball.vx, ball.vy), normal, 0.78);
        ball.vx = reflected.x;
        ball.vy = reflected.y;

        if normal.y < -0.1 {
            ball.vx *= 0.98;
        }

        // Si la collision a lieu sur le haut de la cage
        if is_top_bounce_active {
            let (center_x, center_y, radius) = ball.circle_hitbox();
            if let Some((corrected_center, corrected_velocity)) = crossbar_top_bounce_state(
                vec2(center_x, center_y),
                radius,
                vec2(ball.vx, ball.vy),
                rect,
                side,
            ) {
                ball.x = corrected_center.x - ball.hitbox.offset_x;
                ball.y = corrected_center.y - ball.hitbox.offset_y;
                ball.vx = corrected_velocity.x;
                ball.vy = corrected_velocity.y;
            }
        }
    }
}

fn crossbar_top_bounce_state(
    center: Vec2,
    radius: f32,
    velocity: Vec2,
    top_rect: Rect,
    side: crate::match_arena::GoalSide,
) -> Option<(Vec2, Vec2)> {
    if !rect_circle_overlap(top_rect, center, radius) {
        return None;
    }

    let touching_from_above = center.y <= top_rect.y + top_rect.h * 0.5;
    if !touching_from_above {
        return None;
    }

    let corrected_center = vec2(center.x, top_rect.y - radius - CROSSBAR_TOP_EPSILON);
    let rebound_vy = -velocity.y.abs().max(CROSSBAR_MIN_REBOUND_SPEED);

    let rebound_vx = if velocity.x.abs() < CROSSBAR_STATIC_ESCAPE_VX {
        let horizontal_direction = match side {
            crate::match_arena::GoalSide::Left => 1.0, // But gauche -> pousse vers la droite
            crate::match_arena::GoalSide::Right => -1.0, // But droit -> pousse vers la gauche
        };
        horizontal_direction * CROSSBAR_STATIC_ESCAPE_VX
    } else {
        velocity.x
    };

    Some((corrected_center, vec2(rebound_vx, rebound_vy)))
}

fn circle_rect_collision(ball: &Ball, rect: Rect) -> Option<(Vec2, f32)> {
    let (center_x, center_y, radius) = ball.circle_hitbox();
    circle_rect_collision_at(vec2(center_x, center_y), radius, rect)
}

fn circle_rect_collision_at(center: Vec2, radius: f32, rect: Rect) -> Option<(Vec2, f32)> {
    let closest_x = center.x.clamp(rect.x, rect.right());
    let closest_y = center.y.clamp(rect.y, rect.bottom());
    let dx = center.x - closest_x;
    let dy = center.y - closest_y;
    let distance_sq = dx * dx + dy * dy;

    if distance_sq > radius * radius {
        return None;
    }

    if distance_sq > 0.0001 {
        let distance = distance_sq.sqrt();
        return Some((vec2(dx / distance, dy / distance), radius - distance));
    }

    let left = (center.x - rect.x).abs();
    let right = (rect.right() - center.x).abs();
    let top = (center.y - rect.y).abs();
    let bottom = (rect.bottom() - center.y).abs();
    let min_distance = left.min(right).min(top.min(bottom));

    let normal = if min_distance == left {
        vec2(-1.0, 0.0)
    } else if min_distance == right {
        vec2(1.0, 0.0)
    } else if min_distance == top {
        vec2(0.0, -1.0)
    } else {
        vec2(0.0, 1.0)
    };

    Some((normal, radius + min_distance))
}

fn reflect_velocity(incoming: Vec2, normal: Vec2, damping: f32) -> Vec2 {
    (incoming - 2.0 * incoming.dot(normal) * normal) * damping
}

fn rect_circle_overlap(rect: Rect, center: Vec2, radius: f32) -> bool {
    let closest_x = center.x.clamp(rect.x, rect.right());
    let closest_y = center.y.clamp(rect.y, rect.bottom());
    let dx = center.x - closest_x;
    let dy = center.y - closest_y;

    dx * dx + dy * dy <= radius * radius
}

