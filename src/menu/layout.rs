use super::buttons::MenuButton;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

#[derive(Clone, Copy)]
pub(super) struct MenuLayout {
    pub(super) logo_slot: Rect,
    pub(super) messi_slot: Rect,
    pub(super) ronaldo_slot: Rect,
    play_slot: Rect,
    scoreboard_slot: Rect,
    quit_slot: Rect,
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

        Self {
            logo_slot,
            messi_slot,
            ronaldo_slot,
            play_slot,
            scoreboard_slot,
            quit_slot,
        }
    }

    pub(super) fn button_rect(&self, button: MenuButton) -> Rect {
        match button {
            MenuButton::Play => self.play_slot,
            MenuButton::Scoreboard => self.scoreboard_slot,
            MenuButton::Quit => self.quit_slot,
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
    fn menu_layout_buttons_do_not_overlap() {
        let layout = MenuLayout::from_screen(1000.0, 600.0);

        assert!(layout.play_slot.bottom() <= layout.scoreboard_slot.y);
        assert!(layout.scoreboard_slot.bottom() <= layout.quit_slot.y);
    }
}
