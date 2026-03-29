use super::buttons::MenuButton;
use crate::arcade_ui::scaled_rect;
use crate::physics::{REFERENCE_WIDTH, REFERENCE_HEIGHT};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub(super) struct MenuLayout {
    pub(super) logo_slot: Rect,
    pub(super) messi_slot: Rect,
    pub(super) ronaldo_slot: Rect,
    play_slot: Rect,
    scoreboard_slot: Rect,
    quit_slot: Rect,
    settings_slot: Rect,
}

impl MenuLayout {
    pub(super) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;

        let logo_slot = scaled_rect(285.0, 48.0, 430.0, 210.0, scale_x, scale_y);
        let messi_slot = scaled_rect(0.0, 8.0, 250.0, 290.0, scale_x, scale_y);
        let ronaldo_slot = scaled_rect(760.0, 18.0, 230.0, 250.0, scale_x, scale_y);
        let play_slot = scaled_rect(310.0, 250.0, 380.0, 96.0, scale_x, scale_y);
        let scoreboard_slot = scaled_rect(310.0, 356.0, 380.0, 96.0, scale_x, scale_y);
        let quit_slot = scaled_rect(310.0, 462.0, 380.0, 96.0, scale_x, scale_y);
        let settings_slot = scaled_rect(904.0, 504.0, 72.0, 72.0, scale_x, scale_y);

        Self {
            logo_slot,
            messi_slot,
            ronaldo_slot,
            play_slot,
            scoreboard_slot,
            quit_slot,
            settings_slot,
        }
    }

    pub(super) fn button_rect(&self, button: MenuButton) -> Rect {
        match button {
            MenuButton::Play => self.play_slot,
            MenuButton::Scoreboard => self.scoreboard_slot,
            MenuButton::Quit => self.quit_slot,
            MenuButton::Settings => self.settings_slot,
        }
    }
}
