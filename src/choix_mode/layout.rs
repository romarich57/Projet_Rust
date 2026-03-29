use super::buttons::ModeSelectionButton;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

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

fn scaled_rect(x: f32, y: f32, w: f32, h: f32, scale_x: f32, scale_y: f32) -> Rect {
    Rect::new(x * scale_x, y * scale_y, w * scale_x, h * scale_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_buttons_do_not_overlap() {
        let layout = ModeSelectionLayout::from_screen(1000.0, 600.0);

        assert!(layout.solo_slot.right() <= layout.one_vs_one_slot.x);
    }

    #[test]
    fn characters_do_not_overlap_back_button() {
        let layout = ModeSelectionLayout::from_screen(1000.0, 600.0);

        assert!(layout.materazzi_slot.right() <= layout.back_slot.x);
        assert!(layout.back_slot.right() <= layout.zidane_slot.x);
    }

    #[test]
    fn back_button_stays_centered() {
        let layout = ModeSelectionLayout::from_screen(1000.0, 600.0);
        let center = layout.back_slot.x + layout.back_slot.w * 0.5;

        assert!((center - 500.0).abs() < 0.001);
    }
}
