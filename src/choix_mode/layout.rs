use super::buttons::ModeSelectionButton;
use crate::arcade_ui::scaled_rect;
use crate::physics::{REFERENCE_WIDTH, REFERENCE_HEIGHT};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub(super) struct ModeSelectionLayout {
    pub(super) title_slot: Rect,
    solo_slot: Rect,
    one_vs_one_slot: Rect,
    pub(super) materazzi_slot: Rect,
    pub(super) zidane_slot: Rect,
    back_slot: Rect,
    pub(super) vs_center: Vec2,
    pub(super) vs_font_size: f32,
}

impl ModeSelectionLayout {
    pub(super) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;

        Self {
            title_slot: scaled_rect(290.0, 34.0, 420.0, 70.0, scale_x, scale_y),
            solo_slot: scaled_rect(60.0, 170.0, 390.0, 125.0, scale_x, scale_y),
            one_vs_one_slot: scaled_rect(550.0, 170.0, 390.0, 125.0, scale_x, scale_y),
            materazzi_slot: scaled_rect(55.0, 320.0, 220.0, 240.0, scale_x, scale_y),
            zidane_slot: scaled_rect(725.0, 325.0, 220.0, 235.0, scale_x, scale_y),
            back_slot: scaled_rect(345.0, 495.0, 310.0, 72.0, scale_x, scale_y),
            vs_center: vec2(500.0 * scale_x, 272.0 * scale_y),
            vs_font_size: 62.0 * scale_y.min(scale_x),
        }
    }

    pub(super) fn button_rect(&self, button: ModeSelectionButton) -> Rect {
        match button {
            ModeSelectionButton::Solo => self.solo_slot,
            ModeSelectionButton::OneVsOne => self.one_vs_one_slot,
            ModeSelectionButton::Back => self.back_slot,
        }
    }
}
