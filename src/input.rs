use macroquad::prelude::*;
use crate::models::player::Player;
use crate::models::player::ControlType;

pub fn handle_keyboard(player: &mut Player) {
    let speed = 3.0;

    let (left_key, right_key, jump_key, shoot_key) = match player.control_type {
        ControlType::Player1 => (KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Space),
        ControlType::Player2 => (KeyCode::Q, KeyCode::D, KeyCode::Z, KeyCode::A),
        ControlType::IA => {
            player.vx = 0.0;
            return;
        }
    };

    // Left / Right
    if is_key_down(left_key) {
        player.vx = -speed;
    } else if is_key_down(right_key) {
        player.vx = speed;
    } else {
        player.vx = 0.0;
    }

    // Jump
    if is_key_pressed(jump_key) && player.jump_count < 2 {
        player.vy = if player.jump_count == 0 { -10.0 } else { -8.0 };
        player.jump_count += 1;
    }

    // Shoot trigger
    if is_key_pressed(shoot_key) {
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