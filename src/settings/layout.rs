use crate::physics::{REFERENCE_WIDTH, REFERENCE_HEIGHT};
use crate::settings::data::BindingAction;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct BindingRowLayout {
    pub(crate) label_pos: Vec2,
    pub(crate) value_rect: Rect,
    pub(crate) divider_y: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct SettingsLayout {
    pub(crate) title_slot: Rect,
    pub(crate) header_icon_slot: Rect,
    pub(crate) subtitle_pos: Vec2,
    pub(crate) panel_rect: Rect,
    pub(crate) solo_banner_slot: Rect,
    pub(crate) one_vs_one_banner_slot: Rect,
    pub(crate) solo_rows: [BindingRowLayout; 4],
    pub(crate) player_one_label_pos: Vec2,
    pub(crate) player_two_label_pos: Vec2,
    pub(crate) player_one_rows: [BindingRowLayout; 4],
    pub(crate) player_two_rows: [BindingRowLayout; 4],
    pub(crate) feedback_rect: Rect,
    pub(crate) reset_button_slot: Rect,
    pub(crate) save_button_slot: Rect,
    pub(crate) return_button_slot: Rect,
    pub(crate) confirm_popup_rect: Rect,
    pub(crate) confirm_leave_rect: Rect,
    pub(crate) confirm_stay_rect: Rect,
}

impl SettingsLayout {
    pub(crate) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;
        let panel_rect = Rect::new(
            120.0 * scale_x,
            118.0 * scale_y,
            760.0 * scale_x,
            358.0 * scale_y,
        );

        let solo_row_start_y = panel_rect.y + 60.0 * scale_y;
        let solo_rows = build_rows(
            panel_rect.x + 54.0 * scale_x,
            panel_rect.x + panel_rect.w - 220.0 * scale_x,
            solo_row_start_y,
            31.0 * scale_y,
            scale_x,
        );

        let one_vs_one_row_start_y = panel_rect.y + 246.0 * scale_y;
        let left_label_x = panel_rect.x + 42.0 * scale_x;
        let left_value_x = panel_rect.x + 242.0 * scale_x;
        let right_label_x = panel_rect.x + 406.0 * scale_x;
        let right_value_x = panel_rect.x + 594.0 * scale_x;
        let player_one_rows = build_rows(
            left_label_x,
            left_value_x,
            one_vs_one_row_start_y,
            24.0 * scale_y,
            scale_x,
        );
        let player_two_rows = build_rows(
            right_label_x,
            right_value_x,
            one_vs_one_row_start_y,
            24.0 * scale_y,
            scale_x,
        );

        Self {
            title_slot: Rect::new(
                176.0 * scale_x,
                18.0 * scale_y,
                648.0 * scale_x,
                68.0 * scale_y,
            ),
            header_icon_slot: Rect::new(
                158.0 * scale_x,
                28.0 * scale_y,
                54.0 * scale_x,
                54.0 * scale_y,
            ),
            subtitle_pos: vec2(500.0 * scale_x, 105.0 * scale_y),
            panel_rect,
            solo_banner_slot: Rect::new(
                panel_rect.x + 246.0 * scale_x,
                panel_rect.y + 8.0 * scale_y,
                268.0 * scale_x,
                34.0 * scale_y,
            ),
            one_vs_one_banner_slot: Rect::new(
                panel_rect.x + 220.0 * scale_x,
                panel_rect.y + 188.0 * scale_y,
                320.0 * scale_x,
                36.0 * scale_y,
            ),
            solo_rows,
            player_one_label_pos: vec2(
                (left_label_x + left_value_x + 128.0 * scale_x) * 0.5,
                panel_rect.y + 232.0 * scale_y,
            ),
            player_two_label_pos: vec2(
                (right_label_x + right_value_x + 128.0 * scale_x) * 0.5,
                panel_rect.y + 232.0 * scale_y,
            ),
            player_one_rows,
            player_two_rows,
            feedback_rect: Rect::new(
                180.0 * scale_x,
                486.0 * scale_y,
                640.0 * scale_x,
                28.0 * scale_y,
            ),
            reset_button_slot: Rect::new(
                168.0 * scale_x,
                508.0 * scale_y,
                318.0 * scale_x,
                52.0 * scale_y,
            ),
            save_button_slot: Rect::new(
                536.0 * scale_x,
                508.0 * scale_y,
                214.0 * scale_x,
                52.0 * scale_y,
            ),
            return_button_slot: Rect::new(
                378.0 * scale_x,
                556.0 * scale_y,
                244.0 * scale_x,
                38.0 * scale_y,
            ),
            confirm_popup_rect: Rect::new(
                280.0 * scale_x,
                204.0 * scale_y,
                440.0 * scale_x,
                166.0 * scale_y,
            ),
            confirm_leave_rect: Rect::new(
                314.0 * scale_x,
                304.0 * scale_y,
                172.0 * scale_x,
                42.0 * scale_y,
            ),
            confirm_stay_rect: Rect::new(
                516.0 * scale_x,
                304.0 * scale_y,
                172.0 * scale_x,
                42.0 * scale_y,
            ),
        }
    }

    pub(crate) fn row_for_action(
        rows: &[BindingRowLayout; 4],
        action: BindingAction,
    ) -> BindingRowLayout {
        rows[action as usize]
    }
}

fn build_rows(
    label_x: f32,
    value_x: f32,
    start_y: f32,
    row_height: f32,
    scale_x: f32,
) -> [BindingRowLayout; 4] {
    std::array::from_fn(|index| {
        let y = start_y + index as f32 * row_height;
        BindingRowLayout {
            label_pos: vec2(label_x, y + row_height * 0.72),
            value_rect: Rect::new(value_x, y - 2.0, 128.0 * scale_x, row_height - 5.0),
            divider_y: y + row_height - 6.0,
        }
    })
}

