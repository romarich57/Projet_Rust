use macroquad::prelude::*;
use crate::models::player::Player;

pub fn handle_keyboard(player: &mut Player) {
    let speed = 3.0;

    // Left / Right
    if is_key_down(KeyCode::Left) {
        player.vx = -speed;
    } else if is_key_down(KeyCode::Right) {
        player.vx = speed;
    } else {
        player.vx = 0.0;
    }

    // Jump
    if is_key_pressed(KeyCode::Up) && player.jump_count < 2 {
        player.vy = if player.jump_count == 0 { -10.0 } else { -8.0 };
        player.jump_count += 1;
    }

    // Shoot trigger
    if is_key_pressed(KeyCode::Space) {
        player.is_shooting = true;
    }
}

pub fn update_animations(player: &mut Player) {
    if player.is_shooting {
        player.foot_angle -= 0.2;
        if player.foot_angle < -1.0 {
            player.is_shooting = false;
        }
    } else if player.foot_angle < 0.0 {
        player.foot_angle += 0.1;
    }
}