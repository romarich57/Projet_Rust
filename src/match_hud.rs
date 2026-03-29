use crate::arcade_ui::draw_slot_texture;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;
const HUD_BAR_INNER_PADDING_X: f32 = 16.0;
const HUD_BUTTON_SIZE: f32 = 48.0;
const HUD_BUTTON_GAP: f32 = 10.0;
const HUD_STATE_GAP: f32 = 14.0;
const HUD_STATE_LEFT_MARGIN: f32 = 18.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HudAction {
    TogglePause,
    OpenQuitConfirmation,
    ConfirmQuit,
    CancelQuit,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HudVisualState {
    Running,
    Paused,
    ConfirmQuit,
    Finished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HudButton {
    Pause,
    Quit,
    CancelQuit,
    ConfirmQuit,
}

pub(crate) struct HudAssets {
    pub(crate) pause_icon: Texture2D,
    pub(crate) continue_icon: Texture2D,
    pub(crate) quit_icon: Texture2D,
}

impl HudAssets {
    pub(crate) fn new(
        pause_icon: Texture2D,
        continue_icon: Texture2D,
        quit_icon: Texture2D,
    ) -> Self {
        Self {
            pause_icon,
            continue_icon,
            quit_icon,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HudLayout {
    pub(crate) bar_rect: Rect,
    pub(crate) timer_rect: Rect,
    pub(crate) score_rect: Rect,
    pub(crate) state_rect: Rect,
    pub(crate) quit_button_rect: Rect,
    pub(crate) pause_button_rect: Rect,
    pub(crate) popup_rect: Rect,
    pub(crate) popup_cancel_rect: Rect,
    pub(crate) popup_confirm_rect: Rect,
}

impl HudLayout {
    pub(crate) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;
        let bar_rect = Rect::new(
            140.0 * scale_x,
            12.0 * scale_y,
            720.0 * scale_x,
            78.0 * scale_y,
        );
        let timer_rect = Rect::new(
            172.0 * scale_x,
            20.0 * scale_y,
            150.0 * scale_x,
            58.0 * scale_y,
        );
        let score_rect = Rect::new(
            356.0 * scale_x,
            14.0 * scale_y,
            286.0 * scale_x,
            62.0 * scale_y,
        );

        let button_size = HUD_BUTTON_SIZE * scale_y;
        let inner_padding_x = HUD_BAR_INNER_PADDING_X * scale_x;
        let button_gap = HUD_BUTTON_GAP * scale_x;
        let state_gap = HUD_STATE_GAP * scale_x;

        let pause_button_rect = Rect::new(
            bar_rect.right() - inner_padding_x - button_size,
            bar_rect.y + (bar_rect.h - button_size) * 0.5,
            button_size,
            button_size,
        );
        let quit_button_rect = Rect::new(
            pause_button_rect.x - button_gap - button_size,
            pause_button_rect.y,
            button_size,
            button_size,
        );

        let state_rect_x = score_rect.right() + HUD_STATE_LEFT_MARGIN * scale_x;
        let state_rect_right = quit_button_rect.x - state_gap;
        let state_rect_width = (state_rect_right - state_rect_x).max(0.0);
        let state_rect = Rect::new(
            state_rect_x,
            bar_rect.y + 20.0 * scale_y,
            state_rect_width,
            52.0 * scale_y,
        );

        Self {
            bar_rect,
            timer_rect,
            score_rect,
            state_rect,
            quit_button_rect,
            pause_button_rect,
            popup_rect: Rect::new(
                325.0 * scale_x,
                175.0 * scale_y,
                350.0 * scale_x,
                170.0 * scale_y,
            ),
            popup_cancel_rect: Rect::new(
                365.0 * scale_x,
                265.0 * scale_y,
                112.0 * scale_x,
                56.0 * scale_y,
            ),
            popup_confirm_rect: Rect::new(
                523.0 * scale_x,
                265.0 * scale_y,
                112.0 * scale_x,
                56.0 * scale_y,
            ),
        }
    }

    fn button_at(self, mouse: Vec2, visual_state: HudVisualState) -> Option<HudButton> {
        if visual_state == HudVisualState::Finished {
            return None;
        }

        if visual_state == HudVisualState::ConfirmQuit {
            if self.popup_cancel_rect.contains(mouse) {
                return Some(HudButton::CancelQuit);
            }
            if self.popup_confirm_rect.contains(mouse) {
                return Some(HudButton::ConfirmQuit);
            }
            return None;
        }

        if self.quit_button_rect.contains(mouse) {
            return Some(HudButton::Quit);
        }

        if self.pause_button_rect.contains(mouse) {
            return Some(HudButton::Pause);
        }

        None
    }
}

#[derive(Default)]
pub(crate) struct HudInteractionState {
    hovered: Option<HudButton>,
    pressed: Option<HudButton>,
}

impl HudInteractionState {
    pub(crate) fn update(&mut self, layout: HudLayout, visual_state: HudVisualState) -> HudAction {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered = layout.button_at(mouse, visual_state);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed = self.hovered;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let action = match (self.pressed, self.hovered) {
                (Some(pressed), Some(hovered)) if pressed == hovered => match hovered {
                    HudButton::Pause => HudAction::TogglePause,
                    HudButton::Quit => HudAction::OpenQuitConfirmation,
                    HudButton::CancelQuit => HudAction::CancelQuit,
                    HudButton::ConfirmQuit => HudAction::ConfirmQuit,
                },
                _ => HudAction::None,
            };
            self.pressed = None;
            return action;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed = None;
        }

        HudAction::None
    }
}

pub(crate) fn draw_hud(
    layout: HudLayout,
    interaction: &HudInteractionState,
    assets: &HudAssets,
    visual_state: HudVisualState,
    remaining_seconds: f32,
    score_left: i32,
    score_right: i32,
    state_label: &str,
) {
    draw_rectangle(
        layout.bar_rect.x,
        layout.bar_rect.y,
        layout.bar_rect.w,
        layout.bar_rect.h,
        color_u8!(8, 21, 48, 220),
    );
    draw_rectangle_lines(
        layout.bar_rect.x,
        layout.bar_rect.y,
        layout.bar_rect.w,
        layout.bar_rect.h,
        3.0,
        color_u8!(53, 170, 214, 255),
    );

    let minutes = remaining_seconds.ceil().max(0.0) as i32 / 60;
    let seconds = remaining_seconds.ceil().max(0.0) as i32 % 60;
    let timer_text = format!("{minutes:02}:{seconds:02}");
    let score_text = format!("{score_left} - {score_right}");

    draw_centered_text(
        &timer_text,
        layout.timer_rect.center(),
        34.0,
        color_u8!(255, 239, 184, 255),
    );
    draw_centered_text(&score_text, layout.score_rect.center(), 48.0, WHITE);
    draw_centered_text(
        state_label,
        layout.state_rect.center(),
        24.0,
        color_u8!(214, 234, 248, 255),
    );

    let pause_hovered = interaction.hovered == Some(HudButton::Pause);
    let pause_pressed = interaction.pressed == Some(HudButton::Pause);
    let quit_hovered = interaction.hovered == Some(HudButton::Quit);
    let quit_pressed = interaction.pressed == Some(HudButton::Quit);

    draw_icon_button(
        layout.quit_button_rect,
        &assets.quit_icon,
        quit_hovered,
        quit_pressed,
        color_u8!(114, 88, 42, 210),
    );
    draw_icon_button(
        layout.pause_button_rect,
        if visual_state == HudVisualState::Paused || visual_state == HudVisualState::ConfirmQuit {
            &assets.continue_icon
        } else {
            &assets.pause_icon
        },
        pause_hovered,
        pause_pressed,
        color_u8!(17, 97, 123, 210),
    );

    if visual_state == HudVisualState::ConfirmQuit {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.44),
        );

        draw_rectangle(
            layout.popup_rect.x,
            layout.popup_rect.y,
            layout.popup_rect.w,
            layout.popup_rect.h,
            color_u8!(10, 20, 40, 238),
        );
        draw_rectangle_lines(
            layout.popup_rect.x,
            layout.popup_rect.y,
            layout.popup_rect.w,
            layout.popup_rect.h,
            3.0,
            color_u8!(78, 210, 255, 255),
        );

        draw_centered_text(
            "Quitter le match ?",
            vec2(layout.popup_rect.center().x, layout.popup_rect.y + 48.0),
            32.0,
            WHITE,
        );
        draw_centered_text(
            "La partie en cours sera abandonnee.",
            vec2(layout.popup_rect.center().x, layout.popup_rect.y + 88.0),
            20.0,
            color_u8!(210, 220, 230, 255),
        );

        draw_icon_button(
            layout.popup_cancel_rect,
            &assets.continue_icon,
            interaction.hovered == Some(HudButton::CancelQuit),
            interaction.pressed == Some(HudButton::CancelQuit),
            color_u8!(12, 87, 110, 230),
        );
        draw_icon_button(
            layout.popup_confirm_rect,
            &assets.quit_icon,
            interaction.hovered == Some(HudButton::ConfirmQuit),
            interaction.pressed == Some(HudButton::ConfirmQuit),
            color_u8!(100, 43, 43, 230),
        );

        draw_centered_text(
            "Continuer",
            vec2(
                layout.popup_cancel_rect.center().x,
                layout.popup_cancel_rect.bottom() + 22.0,
            ),
            18.0,
            WHITE,
        );
        draw_centered_text(
            "Quitter",
            vec2(
                layout.popup_confirm_rect.center().x,
                layout.popup_confirm_rect.bottom() + 22.0,
            ),
            18.0,
            WHITE,
        );
    }
}

fn draw_icon_button(rect: Rect, icon: &Texture2D, hovered: bool, pressed: bool, base_color: Color) {
    let fill = match (hovered, pressed) {
        (_, true) => Color::new(
            base_color.r * 0.82,
            base_color.g * 0.82,
            base_color.b * 0.82,
            1.0,
        ),
        (true, false) => Color::new(
            (base_color.r + 0.12).min(1.0),
            (base_color.g + 0.12).min(1.0),
            (base_color.b + 0.12).min(1.0),
            1.0,
        ),
        _ => base_color,
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.5,
        Color::new(1.0, 1.0, 1.0, 0.28),
    );
    draw_slot_texture(icon, rect, if hovered { 0.74 } else { 0.68 });
}

fn draw_centered_text(text: &str, center: Vec2, font_size: f32, color: Color) {
    let metrics = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        center.x - metrics.width * 0.5,
        center.y + metrics.height * 0.35,
        font_size,
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect_contains_rect(outer: Rect, inner: Rect) -> bool {
        inner.x >= outer.x
            && inner.y >= outer.y
            && inner.right() <= outer.right()
            && inner.bottom() <= outer.bottom()
    }

    #[test]
    fn hud_buttons_stay_inside_bar() {
        let layout = HudLayout::from_screen(1000.0, 600.0);

        assert!(rect_contains_rect(layout.bar_rect, layout.quit_button_rect));
        assert!(rect_contains_rect(
            layout.bar_rect,
            layout.pause_button_rect
        ));
    }

    #[test]
    fn quit_button_stays_left_of_pause_button() {
        let layout = HudLayout::from_screen(1000.0, 600.0);

        assert!(layout.quit_button_rect.right() < layout.pause_button_rect.x);
    }

    #[test]
    fn state_rect_stays_before_quit_button_gap() {
        let layout = HudLayout::from_screen(1000.0, 600.0);

        assert!(layout.state_rect.right() <= layout.quit_button_rect.x - HUD_STATE_GAP);
    }

    #[test]
    fn hud_buttons_are_disabled_when_match_is_finished() {
        let layout = HudLayout::from_screen(1000.0, 600.0);
        let pause_center = layout.pause_button_rect.center();

        assert_eq!(
            layout.button_at(pause_center, HudVisualState::Finished),
            None
        );
    }
}
