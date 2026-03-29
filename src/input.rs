use crate::models::player::{Player, MAX_KICK_ANGLE};
use crate::settings::PlayerBindings;
use macroquad::prelude::*;

const KICK_SWING_SPEED: f32 = 0.24;
const KICK_RECOVERY_SPEED: f32 = 0.14;

pub fn handle_keyboard(player: &mut Player, bindings: &PlayerBindings) {
    let speed = 4.0;

    // Left / Right
    if is_key_down(bindings.move_left) {
        player.vx = -speed;
    } else if is_key_down(bindings.move_right) {
        player.vx = speed;
    } else {
        player.vx = 0.0;
    }

    // Jump
    if is_key_pressed(bindings.jump) && player.jump_count < 2 {
        player.vy = if player.jump_count == 0 { -12.0 } else { -9.0 };
        player.jump_count += 1;
    }

    // Shoot trigger
    if is_key_pressed(bindings.shoot) {
        player.is_shooting = true;
    }
}

pub fn update_animations(player: &mut Player) {
    if player.is_shooting {
        let kick_sign = kick_angle_sign(player.side);
        player.foot_angle += kick_sign * KICK_SWING_SPEED;

        if player.foot_angle.abs() >= MAX_KICK_ANGLE {
            player.foot_angle = kick_limit_for_side(player.side);
            player.is_shooting = false;
        }
    } else if player.foot_angle.abs() > 0.0 {
        let recovery = KICK_RECOVERY_SPEED.min(player.foot_angle.abs());
        player.foot_angle -= player.foot_angle.signum() * recovery;
    }
}

fn kick_angle_sign(side: i32) -> f32 {
    if side < 0 {
        -1.0
    } else {
        1.0
    }
}

fn kick_limit_for_side(side: i32) -> f32 {
    kick_angle_sign(side) * MAX_KICK_ANGLE
}

#[cfg(test)]
mod animation_tests {
    use super::*;

    #[test]
    fn left_player_kick_rotates_to_negative_limit() {
        assert_eq!(kick_limit_for_side(-1), -MAX_KICK_ANGLE);
    }

    #[test]
    fn right_player_kick_rotates_to_positive_limit() {
        assert_eq!(kick_limit_for_side(1), MAX_KICK_ANGLE);
    }
}
