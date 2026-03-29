use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;
const MAX_VISIBLE_ROWS: usize = 5;

#[derive(Clone, Copy)]
pub(crate) struct LeaderboardRowLayout {
    pub(crate) rect: Rect,
    pub(crate) mode_slot: Rect,
    pub(crate) score_center_x: f32,
    pub(crate) score_baseline_y: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct LeaderboardLayout {
    pub(crate) victory_slot: Rect,
    pub(crate) victory_value_pos: Vec2,
    pub(crate) defeat_slot: Rect,
    pub(crate) defeat_value_pos: Vec2,
    pub(crate) history_slot: Rect,
    pub(crate) panel_rect: Rect,
    pub(crate) mode_header_slot: Rect,
    pub(crate) score_header_slot: Rect,
    pub(crate) return_button_slot: Rect,
    pub(crate) empty_state_center: Vec2,
    pub(crate) row_layouts: [LeaderboardRowLayout; MAX_VISIBLE_ROWS],
}

impl LeaderboardLayout {
    pub(crate) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;

        let panel_rect = Rect::new(
            246.0 * scale_x,
            214.0 * scale_y,
            510.0 * scale_x,
            266.0 * scale_y,
        );
        let row_height = 36.0 * scale_y;
        let rows_top = panel_rect.y + 74.0 * scale_y;
        let row_width = panel_rect.w - 44.0 * scale_x;
        let row_x = panel_rect.x + 22.0 * scale_x;

        let row_layouts = std::array::from_fn(|index| {
            let y = rows_top + index as f32 * row_height;
            let rect = Rect::new(row_x, y, row_width, row_height);
            LeaderboardRowLayout {
                rect,
                mode_slot: Rect::new(
                    rect.x + 14.0 * scale_x,
                    rect.y + 2.0 * scale_y,
                    122.0 * scale_x,
                    32.0 * scale_y,
                ),
                score_center_x: rect.x + rect.w * 0.79,
                score_baseline_y: rect.y + rect.h * 0.72,
            }
        });

        Self {
            victory_slot: Rect::new(
                80.0 * scale_x,
                34.0 * scale_y,
                372.0 * scale_x,
                116.0 * scale_y,
            ),
            victory_value_pos: vec2(265.0 * scale_x, 163.0 * scale_y),
            defeat_slot: Rect::new(
                548.0 * scale_x,
                34.0 * scale_y,
                372.0 * scale_x,
                116.0 * scale_y,
            ),
            defeat_value_pos: vec2(734.0 * scale_x, 163.0 * scale_y),
            history_slot: Rect::new(
                317.0 * scale_x,
                135.0 * scale_y,
                362.0 * scale_x,
                98.0 * scale_y,
            ),
            panel_rect,
            mode_header_slot: Rect::new(
                panel_rect.x + 47.0 * scale_x,
                panel_rect.y + 6.0 * scale_y,
                155.0 * scale_x,
                62.0 * scale_y,
            ),
            score_header_slot: Rect::new(
                panel_rect.x + panel_rect.w - 205.0 * scale_x,
                panel_rect.y + 6.0 * scale_y,
                155.0 * scale_x,
                62.0 * scale_y,
            ),
            return_button_slot: Rect::new(
                377.0 * scale_x,
                498.0 * scale_y,
                246.0 * scale_x,
                78.0 * scale_y,
            ),
            empty_state_center: vec2(
                panel_rect.x + panel_rect.w * 0.5,
                panel_rect.y + panel_rect.h * 0.61,
            ),
            row_layouts,
        }
    }

    pub(crate) fn visible_rows(&self) -> &[LeaderboardRowLayout] {
        &self.row_layouts
    }
}
