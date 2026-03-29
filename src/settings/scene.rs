use crate::app::SceneCommand;
use crate::arcade_ui::{draw_cover_texture, draw_slot_texture};
use crate::settings::assets::SettingsAssets;
use crate::settings::data::{
    keycode_to_display_label, BindingAction, BindingTarget, ControlProfile, SettingsData,
    SettingsValidationError,
};
use crate::settings::layout::{BindingRowLayout, SettingsLayout};
use macroquad::prelude::*;

const PANEL_FILL: Color = Color::new(0.03, 0.09, 0.17, 0.8);
const PANEL_BORDER: Color = Color::new(0.31, 0.82, 1.0, 0.92);
const PANEL_LINE: Color = Color::new(0.42, 0.76, 1.0, 0.22);
const LABEL_COLOR: Color = Color::new(0.96, 0.88, 0.45, 0.98);
const TEXT_COLOR: Color = Color::new(0.88, 0.94, 1.0, 0.98);
const VALUE_FILL: Color = Color::new(0.04, 0.12, 0.25, 0.9);
const VALUE_BORDER: Color = Color::new(0.35, 0.85, 1.0, 0.95);
const VALUE_ACTIVE_BORDER: Color = Color::new(1.0, 0.84, 0.32, 1.0);
const VALUE_HOVER_BORDER: Color = Color::new(0.6, 0.9, 1.0, 1.0);
const SUCCESS_COLOR: Color = Color::new(0.77, 1.0, 0.69, 0.98);
const ERROR_COLOR: Color = Color::new(1.0, 0.6, 0.55, 0.98);

#[derive(Clone, Debug)]
pub(crate) struct SettingsFeedback {
    text: String,
    color: Color,
}

impl SettingsFeedback {
    pub(crate) fn success(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: SUCCESS_COLOR,
        }
    }

    pub(crate) fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: ERROR_COLOR,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SettingsButton {
    Binding(BindingTarget),
    Reset,
    Save,
    Return,
    ConfirmLeave,
    ConfirmStay,
}

pub(crate) struct SettingsScene {
    background: Texture2D,
    title_banner: Texture2D,
    solo_banner: Texture2D,
    one_vs_one_banner: Texture2D,
    reset_button: Texture2D,
    save_button: Texture2D,
    return_button: Texture2D,
    settings_icon: Texture2D,
    saved: SettingsData,
    draft: SettingsData,
    layout: SettingsLayout,
    hovered_button: Option<SettingsButton>,
    pressed_button: Option<SettingsButton>,
    pending_rebind: Option<BindingTarget>,
    feedback: Option<SettingsFeedback>,
    show_unsaved_confirm: bool,
    last_screen_size: Vec2,
}

impl SettingsScene {
    pub(crate) fn new(assets: &SettingsAssets, saved: SettingsData) -> Self {
        Self::from_state(assets, saved, saved, None)
    }

    pub(crate) fn from_state(
        assets: &SettingsAssets,
        saved: SettingsData,
        draft: SettingsData,
        feedback: Option<SettingsFeedback>,
    ) -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        Self {
            background: assets.background.clone(),
            title_banner: assets.title_banner.clone(),
            solo_banner: assets.solo_banner.clone(),
            one_vs_one_banner: assets.one_vs_one_banner.clone(),
            reset_button: assets.reset_button.clone(),
            save_button: assets.save_button.clone(),
            return_button: assets.return_button.clone(),
            settings_icon: assets.settings_icon.clone(),
            saved,
            draft,
            layout: SettingsLayout::from_screen(screen_size.x, screen_size.y),
            hovered_button: None,
            pressed_button: None,
            pending_rebind: None,
            feedback,
            show_unsaved_confirm: false,
            last_screen_size: screen_size,
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        if self.pending_rebind.is_some() {
            return self.update_rebind_mode();
        }

        if self.show_unsaved_confirm {
            return self.update_confirmation_mode();
        }

        if is_key_pressed(KeyCode::Escape) {
            return self.try_leave();
        }

        self.update_hovered_button();

        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed_button = self.hovered_button;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let clicked = match (self.pressed_button, self.hovered_button) {
                (Some(pressed), Some(hovered)) if pressed == hovered => Some(hovered),
                _ => None,
            };
            self.pressed_button = None;
            if let Some(button) = clicked {
                return self.activate_button(button);
            }
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_button = None;
        }

        SceneCommand::None
    }

    pub(crate) fn draw(&self) {
        draw_cover_texture(
            &self.background,
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
        );
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.01, 0.03, 0.08, 0.36),
        );

        draw_slot_texture(&self.title_banner, self.layout.title_slot, 1.0);
        draw_slot_texture(&self.settings_icon, self.layout.header_icon_slot, 1.0);
        draw_panel(self.layout.panel_rect);
        draw_slot_texture(&self.solo_banner, self.layout.solo_banner_slot, 1.0);
        draw_slot_texture(
            &self.one_vs_one_banner,
            self.layout.one_vs_one_banner_slot,
            1.0,
        );

        draw_shadowed_centered_text(
            "TOUCHES",
            self.layout.subtitle_pos.x,
            self.layout.subtitle_pos.y,
            26.0 * vertical_scale(),
            color_u8!(255, 230, 128, 255),
        );

        self.draw_rows();
        self.draw_feedback();
        self.draw_action_buttons();

        if self.show_unsaved_confirm {
            self.draw_confirmation_popup();
        }
    }

    fn update_rebind_mode(&mut self) -> SceneCommand {
        if is_key_pressed(KeyCode::Escape) {
            self.pending_rebind = None;
            self.feedback = Some(SettingsFeedback::error("Remappage annule"));
            return SceneCommand::None;
        }

        if let Some(target) = self.pending_rebind {
            if let Some(key) = next_pressed_key() {
                match self.try_apply_rebind(target, key) {
                    Ok(()) => {
                        self.pending_rebind = None;
                    }
                    Err(message) => {
                        self.feedback = Some(SettingsFeedback::error(message));
                    }
                }
            }
        }

        SceneCommand::None
    }

    fn update_confirmation_mode(&mut self) -> SceneCommand {
        if is_key_pressed(KeyCode::Escape) {
            self.show_unsaved_confirm = false;
            self.pressed_button = None;
            self.hovered_button = None;
            return SceneCommand::None;
        }

        self.update_hovered_button();

        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed_button = self.hovered_button;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let clicked = match (self.pressed_button, self.hovered_button) {
                (Some(pressed), Some(hovered)) if pressed == hovered => Some(hovered),
                _ => None,
            };
            self.pressed_button = None;
            if let Some(button) = clicked {
                return match button {
                    SettingsButton::ConfirmLeave => SceneCommand::BackToMenu,
                    SettingsButton::ConfirmStay => {
                        self.show_unsaved_confirm = false;
                        SceneCommand::None
                    }
                    _ => SceneCommand::None,
                };
            }
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_button = None;
        }

        SceneCommand::None
    }

    fn activate_button(&mut self, button: SettingsButton) -> SceneCommand {
        self.feedback = None;

        match button {
            SettingsButton::Binding(target) => {
                self.pending_rebind = Some(target);
                SceneCommand::None
            }
            SettingsButton::Reset => {
                self.draft = SettingsData::default();
                self.pending_rebind = None;
                self.feedback = Some(SettingsFeedback::success("Touches par defaut restaurees"));
                SceneCommand::None
            }
            SettingsButton::Save => {
                if !self.can_save() {
                    return SceneCommand::None;
                }

                match self.draft.validate() {
                    Ok(()) => SceneCommand::SaveSettings(self.draft),
                    Err(err) => {
                        self.feedback =
                            Some(SettingsFeedback::error(validation_error_message(err)));
                        SceneCommand::None
                    }
                }
            }
            SettingsButton::Return => self.try_leave(),
            SettingsButton::ConfirmLeave => SceneCommand::BackToMenu,
            SettingsButton::ConfirmStay => {
                self.show_unsaved_confirm = false;
                SceneCommand::None
            }
        }
    }

    fn try_leave(&mut self) -> SceneCommand {
        if self.is_dirty() {
            self.show_unsaved_confirm = true;
            self.pending_rebind = None;
            self.hovered_button = None;
            self.pressed_button = None;
            SceneCommand::None
        } else {
            SceneCommand::BackToMenu
        }
    }

    fn try_apply_rebind(
        &mut self,
        target: BindingTarget,
        key: KeyCode,
    ) -> Result<(), &'static str> {
        if crate::settings::data::is_forbidden_key(key) {
            return Err("Cette touche ne peut pas etre assignee");
        }

        if self
            .draft
            .controls
            .one_vs_one_conflict_for(key, target.profile)
            .is_some()
        {
            return Err("Cette touche est deja utilisee par l'autre joueur");
        }

        self.draft.controls.set_binding(target, key);
        self.feedback = Some(SettingsFeedback::success("Touche mise a jour"));
        Ok(())
    }

    fn update_hovered_button(&mut self) {
        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered_button = if self.show_unsaved_confirm {
            button_in_rect(
                mouse,
                self.layout.confirm_leave_rect,
                SettingsButton::ConfirmLeave,
            )
            .or_else(|| {
                button_in_rect(
                    mouse,
                    self.layout.confirm_stay_rect,
                    SettingsButton::ConfirmStay,
                )
            })
        } else {
            self.binding_button_at(mouse)
                .or_else(|| {
                    button_in_rect(mouse, self.layout.reset_button_slot, SettingsButton::Reset)
                })
                .or_else(|| {
                    button_in_rect(mouse, self.layout.save_button_slot, SettingsButton::Save)
                })
                .or_else(|| {
                    button_in_rect(
                        mouse,
                        self.layout.return_button_slot,
                        SettingsButton::Return,
                    )
                })
        };
    }

    fn binding_button_at(&self, mouse: Vec2) -> Option<SettingsButton> {
        self.iter_binding_rects().find_map(|(target, rect)| {
            rect.contains(mouse)
                .then_some(SettingsButton::Binding(target))
        })
    }

    fn iter_binding_rects(&self) -> impl Iterator<Item = (BindingTarget, Rect)> + '_ {
        iter_profile_rows(ControlProfile::Solo, &self.layout.solo_rows)
            .chain(iter_profile_rows(
                ControlProfile::OneVsOneP1,
                &self.layout.player_one_rows,
            ))
            .chain(iter_profile_rows(
                ControlProfile::OneVsOneP2,
                &self.layout.player_two_rows,
            ))
    }

    fn draw_rows(&self) {
        let solo_scale = vertical_scale();
        self.draw_profile_rows(
            ControlProfile::Solo,
            &self.layout.solo_rows,
            28.0 * solo_scale,
            24.0 * solo_scale,
        );

        draw_shadowed_centered_text(
            ControlProfile::OneVsOneP1.label(),
            self.layout.player_one_label_pos.x,
            self.layout.player_one_label_pos.y,
            24.0 * solo_scale,
            LABEL_COLOR,
        );
        draw_shadowed_centered_text(
            ControlProfile::OneVsOneP2.label(),
            self.layout.player_two_label_pos.x,
            self.layout.player_two_label_pos.y,
            24.0 * solo_scale,
            LABEL_COLOR,
        );

        self.draw_profile_rows(
            ControlProfile::OneVsOneP1,
            &self.layout.player_one_rows,
            20.0 * solo_scale,
            19.0 * solo_scale,
        );
        self.draw_profile_rows(
            ControlProfile::OneVsOneP2,
            &self.layout.player_two_rows,
            20.0 * solo_scale,
            19.0 * solo_scale,
        );
    }

    fn draw_profile_rows(
        &self,
        profile: ControlProfile,
        rows: &[BindingRowLayout; 4],
        label_size: f32,
        value_size: f32,
    ) {
        for action in BindingAction::ALL {
            let row = SettingsLayout::row_for_action(rows, action);
            let target = BindingTarget { profile, action };
            let is_pending = self.pending_rebind == Some(target);
            let is_hovered = self.hovered_button == Some(SettingsButton::Binding(target));

            draw_text(
                action.label(),
                row.label_pos.x,
                row.label_pos.y,
                label_size,
                LABEL_COLOR,
            );
            draw_line(
                row.label_pos.x,
                row.divider_y,
                row.value_rect.right(),
                row.divider_y,
                1.0,
                PANEL_LINE,
            );
            draw_binding_cell(
                row.value_rect,
                &keycode_to_display_label(self.draft.controls.binding(target)),
                value_size,
                is_hovered,
                is_pending,
            );
        }
    }

    fn draw_feedback(&self) {
        if let Some(feedback) = &self.feedback {
            draw_shadowed_centered_text(
                &feedback.text,
                self.layout.feedback_rect.center().x,
                self.layout.feedback_rect.y + self.layout.feedback_rect.h,
                22.0 * vertical_scale(),
                feedback.color,
            );
        } else if self.pending_rebind.is_some() {
            draw_shadowed_centered_text(
                "Appuyez sur une touche...",
                self.layout.feedback_rect.center().x,
                self.layout.feedback_rect.y + self.layout.feedback_rect.h,
                22.0 * vertical_scale(),
                color_u8!(255, 230, 132, 255),
            );
        }
    }

    fn draw_action_buttons(&self) {
        draw_textured_button(
            &self.reset_button,
            self.layout.reset_button_slot,
            self.hovered_button == Some(SettingsButton::Reset),
            self.pressed_button == Some(SettingsButton::Reset),
            true,
        );
        draw_textured_button(
            &self.save_button,
            self.layout.save_button_slot,
            self.hovered_button == Some(SettingsButton::Save),
            self.pressed_button == Some(SettingsButton::Save),
            self.can_save(),
        );
        draw_textured_button(
            &self.return_button,
            self.layout.return_button_slot,
            self.hovered_button == Some(SettingsButton::Return),
            self.pressed_button == Some(SettingsButton::Return),
            true,
        );
    }

    fn draw_confirmation_popup(&self) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.5),
        );
        draw_panel(self.layout.confirm_popup_rect);

        draw_shadowed_centered_text(
            "Quitter sans sauvegarder ?",
            self.layout.confirm_popup_rect.center().x,
            self.layout.confirm_popup_rect.y + 52.0 * vertical_scale(),
            32.0 * vertical_scale(),
            TEXT_COLOR,
        );
        draw_shadowed_centered_text(
            "Les changements non sauvegardes seront perdus.",
            self.layout.confirm_popup_rect.center().x,
            self.layout.confirm_popup_rect.y + 88.0 * vertical_scale(),
            20.0 * vertical_scale(),
            color_u8!(228, 237, 248, 255),
        );

        draw_text_button(
            self.layout.confirm_leave_rect,
            "QUITTER",
            self.hovered_button == Some(SettingsButton::ConfirmLeave),
            self.pressed_button == Some(SettingsButton::ConfirmLeave),
            Color::new(0.39, 0.14, 0.12, 0.9),
        );
        draw_text_button(
            self.layout.confirm_stay_rect,
            "ANNULER",
            self.hovered_button == Some(SettingsButton::ConfirmStay),
            self.pressed_button == Some(SettingsButton::ConfirmStay),
            Color::new(0.05, 0.18, 0.32, 0.9),
        );
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());
        if current_screen_size != self.last_screen_size {
            self.layout = SettingsLayout::from_screen(current_screen_size.x, current_screen_size.y);
            self.last_screen_size = current_screen_size;
        }
    }

    fn is_dirty(&self) -> bool {
        self.saved != self.draft
    }

    fn can_save(&self) -> bool {
        self.pending_rebind.is_none()
            && self.saved != self.draft
            && self.draft.validate().is_ok()
            && !self.show_unsaved_confirm
    }
}

fn iter_profile_rows(
    profile: ControlProfile,
    rows: &[BindingRowLayout; 4],
) -> impl Iterator<Item = (BindingTarget, Rect)> + '_ {
    BindingAction::ALL
        .into_iter()
        .zip(rows.iter().copied())
        .map(move |(action, row)| (BindingTarget { profile, action }, row.value_rect))
}

fn button_in_rect(mouse: Vec2, rect: Rect, button: SettingsButton) -> Option<SettingsButton> {
    rect.contains(mouse).then_some(button)
}

fn next_pressed_key() -> Option<KeyCode> {
    let mut keys: Vec<_> = get_keys_pressed().into_iter().collect();
    keys.sort_by_key(|key| *key as u16);
    keys.into_iter()
        .find(|key| !matches!(key, KeyCode::Unknown))
}

fn validation_error_message(error: SettingsValidationError) -> &'static str {
    match error {
        SettingsValidationError::ForbiddenKey(_) => "Cette touche ne peut pas etre assignee",
        SettingsValidationError::OneVsOneConflict { .. } => {
            "Cette touche est deja utilisee par l'autre joueur"
        }
    }
}

fn draw_panel(rect: Rect) {
    draw_rectangle(
        rect.x + 6.0,
        rect.y + 8.0,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.28),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, PANEL_FILL);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, PANEL_BORDER);
}

fn draw_binding_cell(rect: Rect, label: &str, font_size: f32, hovered: bool, active: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, VALUE_FILL);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        if active {
            VALUE_ACTIVE_BORDER
        } else if hovered {
            VALUE_HOVER_BORDER
        } else {
            VALUE_BORDER
        },
    );

    let text_color = if active {
        color_u8!(255, 230, 128, 255)
    } else {
        TEXT_COLOR
    };

    if let Some(direction) = direction_from_label(label) {
        draw_directional_key(rect, direction, text_color);
    } else {
        let adjusted_font_size = if label.len() > 4 {
            font_size * 0.82
        } else {
            font_size
        };
        draw_shadowed_centered_text(
            label,
            rect.center().x,
            rect.y + rect.h * 0.72,
            adjusted_font_size,
            text_color,
        );
    }
}

#[derive(Clone, Copy)]
enum DirectionArrow {
    Left,
    Right,
    Up,
    Down,
}

fn direction_from_label(label: &str) -> Option<DirectionArrow> {
    match label {
        "LEFT" => Some(DirectionArrow::Left),
        "RIGHT" => Some(DirectionArrow::Right),
        "UP" => Some(DirectionArrow::Up),
        "DOWN" => Some(DirectionArrow::Down),
        _ => None,
    }
}

fn draw_directional_key(rect: Rect, direction: DirectionArrow, color: Color) {
    let stroke = rect.h * 0.16;
    let center = rect.center();
    let max_arrow_width = rect.h * 1.6;
    let h_padding = (rect.w - max_arrow_width).max(rect.h * 0.4) * 0.5;
    let v_padding = rect.h * 0.22;
    let shadow = Color::new(0.02, 0.04, 0.08, 0.86);

    match direction {
        DirectionArrow::Left => draw_arrow_segment(
            vec2(rect.right() - h_padding, center.y),
            vec2(rect.x + h_padding, center.y),
            stroke,
            color,
            shadow,
        ),
        DirectionArrow::Right => draw_arrow_segment(
            vec2(rect.x + h_padding, center.y),
            vec2(rect.right() - h_padding, center.y),
            stroke,
            color,
            shadow,
        ),
        DirectionArrow::Up => draw_arrow_segment(
            vec2(center.x, rect.bottom() - v_padding),
            vec2(center.x, rect.y + v_padding),
            stroke,
            color,
            shadow,
        ),
        DirectionArrow::Down => draw_arrow_segment(
            vec2(center.x, rect.y + v_padding),
            vec2(center.x, rect.bottom() - v_padding),
            stroke,
            color,
            shadow,
        ),
    }
}

fn draw_arrow_segment(start: Vec2, end: Vec2, thickness: f32, color: Color, shadow: Color) {
    let direction = (end - start).normalize_or_zero();
    let perpendicular = vec2(-direction.y, direction.x);
    let head_size = 12.0_f32.max(thickness * 3.0);
    let shadow_offset = vec2(1.5, 1.5);
    let shaft_end = end - direction * head_size * 0.9;

    draw_line(
        start.x + shadow_offset.x,
        start.y + shadow_offset.y,
        shaft_end.x + shadow_offset.x,
        shaft_end.y + shadow_offset.y,
        thickness,
        shadow,
    );
    draw_line(start.x, start.y, shaft_end.x, shaft_end.y, thickness, color);

    let head_left = end - direction * head_size + perpendicular * (head_size * 0.55);
    let head_right = end - direction * head_size - perpendicular * (head_size * 0.55);

    draw_line(
        end.x + shadow_offset.x,
        end.y + shadow_offset.y,
        head_left.x + shadow_offset.x,
        head_left.y + shadow_offset.y,
        thickness,
        shadow,
    );
    draw_line(
        end.x + shadow_offset.x,
        end.y + shadow_offset.y,
        head_right.x + shadow_offset.x,
        head_right.y + shadow_offset.y,
        thickness,
        shadow,
    );
    draw_line(end.x, end.y, head_left.x, head_left.y, thickness, color);
    draw_line(end.x, end.y, head_right.x, head_right.y, thickness, color);
}

fn draw_textured_button(
    texture: &Texture2D,
    slot: Rect,
    hovered: bool,
    pressed: bool,
    enabled: bool,
) {
    let scale = if pressed {
        0.98
    } else if hovered && enabled {
        1.03
    } else {
        1.0
    };

    let tint = if enabled {
        WHITE
    } else {
        Color::new(1.0, 1.0, 1.0, 0.42)
    };

    let base_rect = crate::arcade_ui::fit_contain(slot, texture.width(), texture.height());
    let draw_rect = crate::arcade_ui::scale_rect_from_center(base_rect, scale);
    draw_texture_ex(
        texture,
        draw_rect.x,
        draw_rect.y,
        tint,
        DrawTextureParams {
            dest_size: Some(vec2(draw_rect.w, draw_rect.h)),
            ..Default::default()
        },
    );
}

fn draw_text_button(rect: Rect, label: &str, hovered: bool, pressed: bool, fill: Color) {
    let stroke = if hovered {
        color_u8!(255, 230, 128, 255)
    } else {
        PANEL_BORDER
    };
    let y_offset = if pressed { 2.0 } else { 0.0 };

    draw_rectangle(rect.x, rect.y + y_offset, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y + y_offset, rect.w, rect.h, 2.0, stroke);
    draw_shadowed_centered_text(
        label,
        rect.center().x,
        rect.y + y_offset + rect.h * 0.68,
        22.0 * vertical_scale(),
        TEXT_COLOR,
    );
}

fn draw_shadowed_centered_text(
    text: &str,
    center_x: f32,
    baseline_y: f32,
    font_size: f32,
    color: Color,
) {
    let metrics = measure_text(text, None, font_size as u16, 1.0);
    let draw_x = center_x - metrics.width * 0.5;
    draw_text(
        text,
        draw_x + 2.0,
        baseline_y + 2.0,
        font_size,
        Color::new(0.02, 0.04, 0.08, 0.86),
    );
    draw_text(text, draw_x, baseline_y, font_size, color);
}

fn vertical_scale() -> f32 {
    screen_height() / 600.0
}
