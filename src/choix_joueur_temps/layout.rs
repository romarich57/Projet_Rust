use super::controls::MatchSetupControl;
use crate::gameplay::MatchMode;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

#[derive(Clone, Copy)]
pub(super) struct MatchSetupLayout {
    pub(super) title_slot: Rect,
    pub(super) left_card: Rect,
    pub(super) right_card: Rect,
    pub(super) left_portrait_slot: Rect,
    pub(super) right_portrait_slot: Rect,
    pub(super) left_name_baseline: f32,
    pub(super) right_name_baseline: f32,
    pub(super) left_prev_arrow: Rect,
    pub(super) left_next_arrow: Rect,
    pub(super) right_prev_arrow: Rect,
    pub(super) right_next_arrow: Rect,
    pub(super) duration_panel: Rect,
    pub(super) duration_title_center: Vec2,
    pub(super) duration_prev_arrow: Rect,
    pub(super) duration_next_arrow: Rect,
    pub(super) duration_value_rect: Rect,
    pub(super) difficulty_rects: Option<[Rect; 3]>,
    back_slot: Rect,
    play_slot: Rect,
}

impl MatchSetupLayout {
    pub(super) fn from_screen(mode: MatchMode, screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;

        let left_card = if mode.shows_difficulty() {
            scaled_rect(220.0, 108.0, 240.0, 245.0, scale_x, scale_y)
        } else {
            scaled_rect(220.0, 120.0, 240.0, 265.0, scale_x, scale_y)
        };
        let right_card = if mode.shows_difficulty() {
            scaled_rect(540.0, 108.0, 240.0, 245.0, scale_x, scale_y)
        } else {
            scaled_rect(540.0, 120.0, 240.0, 265.0, scale_x, scale_y)
        };

        let duration_panel = if mode.shows_difficulty() {
            scaled_rect(348.0, 364.0, 304.0, 84.0, scale_x, scale_y)
        } else {
            scaled_rect(348.0, 402.0, 304.0, 92.0, scale_x, scale_y)
        };

        let difficulty_rects = mode.shows_difficulty().then(|| {
            let origin_x = 282.0;
            let origin_y = 448.0;
            let width = 136.0;
            let gap = 14.0;
            let height = 46.0;

            [
                scaled_rect(origin_x, origin_y, width, height, scale_x, scale_y),
                scaled_rect(
                    origin_x + width + gap,
                    origin_y,
                    width,
                    height,
                    scale_x,
                    scale_y,
                ),
                scaled_rect(
                    origin_x + (width + gap) * 2.0,
                    origin_y,
                    width,
                    height,
                    scale_x,
                    scale_y,
                ),
            ]
        });

        Self {
            title_slot: scaled_rect(300.0, 24.0, 400.0, 72.0, scale_x, scale_y),
            left_card,
            right_card,
            left_portrait_slot: inset_rect(left_card, 34.0, 28.0, 34.0, left_card.h - 135.0),
            right_portrait_slot: inset_rect(right_card, 34.0, 28.0, 34.0, right_card.h - 135.0),
            left_name_baseline: left_card.y + left_card.h - 48.0 * scale_y,
            right_name_baseline: right_card.y + right_card.h - 48.0 * scale_y,
            left_prev_arrow: Rect::new(
                left_card.x + 18.0 * scale_x,
                left_card.y + left_card.h - 80.0 * scale_y,
                48.0 * scale_x,
                48.0 * scale_y,
            ),
            left_next_arrow: Rect::new(
                left_card.x + left_card.w - 66.0 * scale_x,
                left_card.y + left_card.h - 80.0 * scale_y,
                48.0 * scale_x,
                48.0 * scale_y,
            ),
            right_prev_arrow: Rect::new(
                right_card.x + 18.0 * scale_x,
                right_card.y + right_card.h - 80.0 * scale_y,
                48.0 * scale_x,
                48.0 * scale_y,
            ),
            right_next_arrow: Rect::new(
                right_card.x + right_card.w - 66.0 * scale_x,
                right_card.y + right_card.h - 80.0 * scale_y,
                48.0 * scale_x,
                48.0 * scale_y,
            ),
            duration_panel,
            duration_title_center: vec2(
                duration_panel.x + duration_panel.w * 0.5,
                duration_panel.y + 20.0 * scale_y,
            ),
            duration_prev_arrow: Rect::new(
                duration_panel.x + 16.0 * scale_x,
                duration_panel.y + duration_panel.h - 54.0 * scale_y,
                46.0 * scale_x,
                46.0 * scale_y,
            ),
            duration_next_arrow: Rect::new(
                duration_panel.x + duration_panel.w - 62.0 * scale_x,
                duration_panel.y + duration_panel.h - 54.0 * scale_y,
                46.0 * scale_x,
                46.0 * scale_y,
            ),
            duration_value_rect: Rect::new(
                duration_panel.x + 78.0 * scale_x,
                duration_panel.y + duration_panel.h - 54.0 * scale_y,
                duration_panel.w - 156.0 * scale_x,
                46.0 * scale_y,
            ),
            difficulty_rects,
            back_slot: scaled_rect(245.0, 514.0, 210.0, 62.0, scale_x, scale_y),
            play_slot: scaled_rect(545.0, 498.0, 255.0, 82.0, scale_x, scale_y),
        }
    }

    pub(super) fn control_at(&self, point: Vec2, mode: MatchMode) -> Option<MatchSetupControl> {
        let controls = [
            MatchSetupControl::LeftPlayerPrev,
            MatchSetupControl::LeftPlayerNext,
            MatchSetupControl::RightPlayerPrev,
            MatchSetupControl::RightPlayerNext,
            MatchSetupControl::LengthPrev,
            MatchSetupControl::LengthNext,
            MatchSetupControl::Back,
            MatchSetupControl::Play,
        ];

        for control in controls {
            if self
                .rect_for_control(control)
                .is_some_and(|rect| rect.contains(point))
            {
                return Some(control);
            }
        }

        if mode.shows_difficulty() {
            for control in [
                MatchSetupControl::DifficultyEasy,
                MatchSetupControl::DifficultyNormal,
                MatchSetupControl::DifficultyHard,
            ] {
                if self
                    .rect_for_control(control)
                    .is_some_and(|rect| rect.contains(point))
                {
                    return Some(control);
                }
            }
        }

        None
    }

    pub(super) fn rect_for_control(&self, control: MatchSetupControl) -> Option<Rect> {
        match control {
            MatchSetupControl::LeftPlayerPrev => Some(self.left_prev_arrow),
            MatchSetupControl::LeftPlayerNext => Some(self.left_next_arrow),
            MatchSetupControl::RightPlayerPrev => Some(self.right_prev_arrow),
            MatchSetupControl::RightPlayerNext => Some(self.right_next_arrow),
            MatchSetupControl::LengthPrev => Some(self.duration_prev_arrow),
            MatchSetupControl::LengthNext => Some(self.duration_next_arrow),
            MatchSetupControl::DifficultyEasy => self.difficulty_rects.map(|rects| rects[0]),
            MatchSetupControl::DifficultyNormal => self.difficulty_rects.map(|rects| rects[1]),
            MatchSetupControl::DifficultyHard => self.difficulty_rects.map(|rects| rects[2]),
            MatchSetupControl::Back => Some(self.back_slot),
            MatchSetupControl::Play => Some(self.play_slot),
        }
    }
}

fn scaled_rect(x: f32, y: f32, w: f32, h: f32, scale_x: f32, scale_y: f32) -> Rect {
    Rect::new(x * scale_x, y * scale_y, w * scale_x, h * scale_y)
}

fn inset_rect(rect: Rect, left: f32, top: f32, right: f32, height: f32) -> Rect {
    Rect::new(rect.x + left, rect.y + top, rect.w - left - right, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_zone_is_absent_in_one_vs_one() {
        let layout = MatchSetupLayout::from_screen(MatchMode::OneVsOne, 1000.0, 600.0);

        assert!(layout.difficulty_rects.is_none());
    }

    #[test]
    fn player_cards_do_not_overlap() {
        let layout = MatchSetupLayout::from_screen(MatchMode::Solo, 1000.0, 600.0);

        assert!(layout.left_card.right() <= layout.right_card.x);
    }

    #[test]
    fn duration_panel_stays_above_bottom_buttons() {
        let layout = MatchSetupLayout::from_screen(MatchMode::OneVsOne, 1000.0, 600.0);

        assert!(layout.duration_panel.y + layout.duration_panel.h <= layout.back_slot.y);
        assert!(layout.duration_panel.y + layout.duration_panel.h <= layout.play_slot.y);
    }

    #[test]
    fn solo_difficulty_row_does_not_overlap_bottom_buttons() {
        let layout = MatchSetupLayout::from_screen(MatchMode::Solo, 1000.0, 600.0);
        let rects = layout
            .difficulty_rects
            .expect("solo should show difficulty");

        for rect in rects {
            assert!(rect.y + rect.h <= layout.back_slot.y);
            assert!(rect.y + rect.h <= layout.play_slot.y);
        }
    }
}
