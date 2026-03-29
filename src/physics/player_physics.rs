use crate::match_arena::ArenaGeometry;
use crate::models::player::Player;
use crate::physics::PLAYER_GRAVITY_REFERENCE;
use macroquad::time::get_frame_time;

pub fn apply_physics(player: &mut Player, arena: &ArenaGeometry) {
    let player_ground_y = player.y_at_ground(arena.ground_y);

    // Framerate independence: 75 FPS reference keeps existing tuning
    let dt = get_frame_time().clamp(1.0 / 240.0, 1.0 / 20.0);
    let scale = dt * 75.0;

    player.x += player.vx * scale;
    player.y += player.vy * scale;

    // Player gravity
    player.vy += PLAYER_GRAVITY_REFERENCE * scale;

    // Ground collision
    if player.y > player_ground_y {
        player.y = player_ground_y;
        player.vy = 0.0;
        player.jump_count = 0;
    }

    // Left wall
    if player.x < 0.0 {
        player.x = arena.player_left_wall_x;
        player.vx = 0.0;
    }

    // Right wall
    let player_width = player.collision_width();
    if player.x < arena.player_left_wall_x {
        player.x = arena.player_left_wall_x;
        player.vx = 0.0;
    }
    if player.x > arena.player_right_wall_x - player_width {
        player.x = arena.player_right_wall_x - player_width;
        player.vx = 0.0;
    }

    // Ceiling
    if player.y < 0.0 {
        player.y = 0.0;
        player.vy = 0.0;
    }
}
