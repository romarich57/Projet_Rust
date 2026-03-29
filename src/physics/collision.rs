use crate::models::ball::Ball;
use crate::models::player::Player;

/// Player-ball collision:
/// - head/body contact (light impulse)
/// - shooting foot contact (strong impulse)
pub fn apply_player_ball_collision(player: &Player, ball: &mut Ball) {
    let (foot_x, foot_y, foot_w, foot_h) = player.active_foot_hitbox_rect();
    let (head_x, head_y, head_w, head_h) = player.head_hitbox_rect();

    let (bcx, bcy, bcr) = ball.circle_hitbox();

    if let Some((nx, ny, penetration)) =
        rect_circle_collision(head_x, head_y, head_w, head_h, bcx, bcy, bcr)
    {
        // Separate shapes to avoid sticky overlap.
        ball.x += nx * penetration;
        ball.y += ny * penetration;

        let force = 3.0;
        ball.vx += nx * force + player.vx * 0.30;

        // Always add slight upward lift on body contact.
        ball.vy += ny * force + player.vy * 0.15;
        ball.vy -= 1.2;
        if ball.vy > -2.6 {
            ball.vy = -2.6;
        }
    }

    if let Some((nx, ny, penetration)) =
        rect_circle_collision(foot_x, foot_y, foot_w, foot_h, bcx, bcy, bcr)
    {
        ball.x += nx * penetration;
        ball.y += ny * penetration;

        let shot_progress = player.shot_progress();
        let in_shot_phase = player.is_shooting || shot_progress > 0.22;

        if in_shot_phase {
            // Higher kick angle and higher contact point create more loft.
            let contact_y = ((bcy - foot_y) / foot_h).clamp(0.0, 1.0);
            let lob_bonus = (1.0 - contact_y) * 0.35;

            // Kick direction depends on which player is kicking
            let expected_dir = -player.side as f32; // -1 -> 1 (Right), 1 -> -1 (Left)
            let dir_x = if nx.abs() > 0.05 { nx } else { expected_dir };
            let dir_y = -(0.35 + 0.70 * shot_progress + lob_bonus);

            let force = 8.5 + 6.5 * shot_progress;
            let speed_transfer = player.vx * 0.45;

            ball.vx += dir_x * force + speed_transfer;
            ball.vy += dir_y * force + player.vy * 0.10;

            // Ensure a minimum upward velocity on a real shot.
            let vy_min = -(3.8 + 2.6 * shot_progress);
            if ball.vy > vy_min {
                ball.vy = vy_min;
            }
        } else {
            // Soft touch outside shot phase: damp speed and add mild push.
            let soft_force = 1.6;
            ball.vx *= 0.86;
            ball.vy *= 0.90;

            ball.vx += nx * soft_force + player.vx * 0.20;
            ball.vy += ny * (soft_force * 0.55);
            ball.vy -= 0.35;
        }
    }

    // Body collision: covers the torso between head and feet.
    let (body_x, body_y, body_w, body_h) = player.body_hitbox_rect();
    if body_h > 0.0 {
        if let Some((nx, ny, penetration)) =
            rect_circle_collision(body_x, body_y, body_w, body_h, bcx, bcy, bcr)
        {
            // Push ball fully out of the body
            ball.x += nx * penetration;
            ball.y += ny * penetration;

            // Gentle deflection: mostly sideways, slight upward lift
            let deflect_force = 2.5;
            ball.vx += nx * deflect_force + player.vx * 0.25;
            ball.vy += ny * deflect_force * 0.5;
            ball.vy -= 0.8;
            if ball.vy > -1.5 {
                ball.vy = -1.5;
            }
        }
    }

    limit_ball_speed(ball, 18.0);
}

fn rect_circle_collision(
    rx: f32,
    ry: f32,
    rw: f32,
    rh: f32,
    cx: f32,
    cy: f32,
    cr: f32,
) -> Option<(f32, f32, f32)> {
    let closest_x = cx.clamp(rx, rx + rw);
    let closest_y = cy.clamp(ry, ry + rh);
    let dx = cx - closest_x;
    let dy = cy - closest_y;
    let dist2 = dx * dx + dy * dy;

    if dist2 > cr * cr {
        return None;
    }

    if dist2 > 0.0001 {
        let dist = dist2.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;
        let penetration = cr - dist;
        return Some((nx, ny, penetration));
    }

    // Circle center inside rectangle: push toward nearest edge.
    let dist_left = (cx - rx).abs();
    let dist_right = (rx + rw - cx).abs();
    let dist_top = (cy - ry).abs();
    let dist_bottom = (ry + rh - cy).abs();

    let min_dist = dist_left.min(dist_right).min(dist_top.min(dist_bottom));

    let (nx, ny) = if min_dist == dist_left {
        (-1.0, 0.0)
    } else if min_dist == dist_right {
        (1.0, 0.0)
    } else if min_dist == dist_top {
        (0.0, -1.0)
    } else {
        (0.0, 1.0)
    };

    let penetration = cr + min_dist;

    Some((nx, ny, penetration))
}

fn limit_ball_speed(ball: &mut Ball, vmax: f32) {
    let speed2 = ball.vx * ball.vx + ball.vy * ball.vy;
    let vmax2 = vmax * vmax;

    if speed2 > vmax2 {
        let speed = speed2.sqrt();
        let scale = vmax / speed;
        ball.vx *= scale;
        ball.vy *= scale;
    }
}

pub fn apply_player_player_collision(p1: &mut Player, p2: &mut Player) {
    // Prevent players from walking through each other
    let hw1 = p1.collision_width() / 3.0; // Use a fraction of the collision width for more forgiving collisions
    let hw2 = p2.collision_width() / 3.0;

    let c1 = p1.x + hw1;
    let c2 = p2.x + hw2;

    let dx = c2 - c1;
    let min_dist = hw1 + hw2 - 1.0; // small leniency

    if dx.abs() < min_dist {
        let overlap = min_dist - dx.abs();
        let push = overlap / 2.0;

        if dx > 0.0 {
            // p2 is mostly to the right
            p1.x -= push;
            p2.x += push;
        } else {
            // p2 is mostly to the left
            p1.x += push;
            p2.x -= push;
        }

        // Dampen relative horizontal velocity slightly to avoid sliding effects
        let avg_vx = (p1.vx + p2.vx) * 0.5;
        p1.vx = avg_vx;
        p2.vx = avg_vx;
    }
}
