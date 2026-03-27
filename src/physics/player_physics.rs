use crate::models::player::Player;
use macroquad::prelude::screen_width;
use macroquad::time::get_frame_time;
use crate::physics::{ground_level, PLAYER_GRAVITY_REFERENCE};


pub fn apply_physics(player: &mut Player) {
    let ground_y = ground_level();
    let player_ground_y = player.y_at_ground(ground_y);

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
        player.x = 0.0;
        player.vx = 0.0;
    }

    // Right wall
    let player_width = player.collision_width();
    if player.x > screen_width() - player_width {
        player.x = screen_width() - player_width;
        player.vx = 0.0;
    }

    // Ceiling
    if player.y < 0.0 {
        player.y = 0.0;
        player.vy = 0.0;
    }

}