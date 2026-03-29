use crate::models::player::ControlType;
use crate::models::player::{Player, MAX_KICK_ANGLE};
use macroquad::prelude::*;

const KICK_SWING_SPEED: f32 = 0.24;
const KICK_RECOVERY_SPEED: f32 = 0.14;

fn bindings_for_control(control_type: ControlType) -> Option<(KeyCode, KeyCode, KeyCode, KeyCode)> {
    match control_type {
        ControlType::Player1 => Some((KeyCode::Q, KeyCode::D, KeyCode::Z, KeyCode::S)),
        ControlType::Player2 => Some((
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::RightControl,
        )),
        ControlType::IA => None,
    }
}

pub fn handle_keyboard(player: &mut Player) {
    let speed = 3.0;

    let (left_key, right_key, jump_key, shoot_key) = match bindings_for_control(player.control_type)
    {
        Some(bindings) => bindings,
        None => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_one_bindings_match_local_mode_layout() {
        assert_eq!(
            bindings_for_control(ControlType::Player1),
            Some((KeyCode::Q, KeyCode::D, KeyCode::Z, KeyCode::S))
        );
    }

    #[test]
    fn player_two_bindings_match_local_mode_layout() {
        assert_eq!(
            bindings_for_control(ControlType::Player2),
            Some((
                KeyCode::Left,
                KeyCode::Right,
                KeyCode::Up,
                KeyCode::RightControl,
            ))
        );
    }

    #[test]
    fn ai_has_no_keyboard_bindings() {
        assert_eq!(bindings_for_control(ControlType::IA), None);
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
