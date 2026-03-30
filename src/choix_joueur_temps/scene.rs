use super::{
    assets::MatchSetupAssets,
    controls::{left_portrait_texture, right_portrait_texture, ArrowDirection, MatchSetupControl},
    layout::MatchSetupLayout,
    widgets::{
        draw_arrow_button, draw_difficulty_button, draw_inner_display, draw_neon_panel,
        draw_text_centered,
    },
};
use crate::app::SceneCommand;
use crate::arcade_ui::draw_slot_texture;
use crate::gameplay::{BotDifficulty, MatchConfig, MatchMode};
use macroquad::prelude::*;

pub(crate) struct MatchSetupScene {
    config: MatchConfig,
    layout: MatchSetupLayout,
    hovered_control: Option<MatchSetupControl>,
    pressed_control: Option<MatchSetupControl>,
    last_screen_size: Vec2,
}

impl MatchSetupScene {
    pub(crate) fn new(mode: MatchMode) -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        let config = MatchConfig::default_for_mode(mode);

        Self {
            layout: MatchSetupLayout::from_screen(mode, screen_size.x, screen_size.y),
            config,
            hovered_control: None,
            pressed_control: None,
            last_screen_size: screen_size,
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        if is_key_pressed(KeyCode::Escape) {
            return SceneCommand::BackToModeSelection;
        }

        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered_control = self.layout.control_at(mouse, self.config.mode);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed_control = self.hovered_control;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let command = match (self.pressed_control, self.hovered_control) {
                (Some(pressed), Some(hovered)) if pressed == hovered => {
                    self.activate_control(hovered)
                }
                _ => SceneCommand::None,
            };
            self.pressed_control = None;
            return command;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_control = None;
        }

        SceneCommand::None
    }

    pub(crate) fn draw(&self, assets: &MatchSetupAssets) {
        draw_texture_ex(
            &assets.background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.24),
        );

        draw_slot_texture(&assets.title, self.layout.title_slot, 1.0);

        self.draw_player_card(
            self.layout.left_card,
            self.layout.left_portrait_slot,
            self.layout.left_name_baseline,
            self.layout.left_prev_arrow,
            self.layout.left_next_arrow,
            self.config.left_player.display_name(),
            left_portrait_texture(assets, self.config.left_player),
            MatchSetupControl::LeftPlayerPrev,
            MatchSetupControl::LeftPlayerNext,
        );
        self.draw_player_card(
            self.layout.right_card,
            self.layout.right_portrait_slot,
            self.layout.right_name_baseline,
            self.layout.right_prev_arrow,
            self.layout.right_next_arrow,
            self.config.right_player.display_name(),
            right_portrait_texture(assets, self.config.right_player),
            MatchSetupControl::RightPlayerPrev,
            MatchSetupControl::RightPlayerNext,
        );

        self.draw_duration_panel();

        if self.config.mode.shows_difficulty() {
            self.draw_difficulty_row();
        }

        self.draw_asset_button(MatchSetupControl::Back, &assets.back_button);
        self.draw_asset_button(MatchSetupControl::Play, &assets.play_button);
    }

    fn activate_control(&mut self, control: MatchSetupControl) -> SceneCommand {
        match control {
            MatchSetupControl::LeftPlayerPrev => {
                self.config.left_player = self.config.left_player.previous();
            }
            MatchSetupControl::LeftPlayerNext => {
                self.config.left_player = self.config.left_player.next();
            }
            MatchSetupControl::RightPlayerPrev => {
                self.config.right_player = self.config.right_player.previous();
            }
            MatchSetupControl::RightPlayerNext => {
                self.config.right_player = self.config.right_player.next();
            }
            MatchSetupControl::LengthPrev => {
                self.config.length = self.config.length.previous();
            }
            MatchSetupControl::LengthNext => {
                self.config.length = self.config.length.next();
            }
            MatchSetupControl::DifficultyEasy => {
                self.config.difficulty = Some(BotDifficulty::Easy);
            }
            MatchSetupControl::DifficultyNormal => {
                self.config.difficulty = Some(BotDifficulty::Normal);
            }
            MatchSetupControl::DifficultyHard => {
                self.config.difficulty = Some(BotDifficulty::Hard);
            }
            MatchSetupControl::Back => return SceneCommand::BackToModeSelection,
            MatchSetupControl::Play => return SceneCommand::StartMatch(self.config),
        }

        SceneCommand::None
    }

    fn draw_player_card(
        &self,
        card: Rect,
        portrait_slot: Rect,
        name_baseline: f32,
        prev_arrow: Rect,
        next_arrow: Rect,
        name: &str,
        texture: &Texture2D,
        prev_control: MatchSetupControl,
        next_control: MatchSetupControl,
    ) {
        draw_neon_panel(card);
        draw_slot_texture(texture, portrait_slot, 1.0);
        draw_arrow_button(
            prev_arrow,
            ArrowDirection::Left,
            self.hovered_control == Some(prev_control),
            self.pressed_control == Some(prev_control),
        );
        draw_arrow_button(
            next_arrow,
            ArrowDirection::Right,
            self.hovered_control == Some(next_control),
            self.pressed_control == Some(next_control),
        );
        draw_text_centered(
            name,
            vec2(card.x + card.w * 0.5, name_baseline),
            32.0,
            WHITE,
        );
    }

    fn draw_duration_panel(&self) {
        draw_neon_panel(self.layout.duration_panel);
        draw_text_centered(
            "DUREE DU MATCH",
            self.layout.duration_title_center,
            18.0,
            color_u8!(238, 216, 130, 255),
        );

        draw_arrow_button(
            self.layout.duration_prev_arrow,
            ArrowDirection::Left,
            self.hovered_control == Some(MatchSetupControl::LengthPrev),
            self.pressed_control == Some(MatchSetupControl::LengthPrev),
        );
        draw_arrow_button(
            self.layout.duration_next_arrow,
            ArrowDirection::Right,
            self.hovered_control == Some(MatchSetupControl::LengthNext),
            self.pressed_control == Some(MatchSetupControl::LengthNext),
        );

        draw_inner_display(self.layout.duration_value_rect);
        draw_text_centered(
            self.config.length.label(),
            vec2(
                self.layout.duration_value_rect.x + self.layout.duration_value_rect.w * 0.5,
                self.layout.duration_value_rect.y + self.layout.duration_value_rect.h * 0.68,
            ),
            44.0,
            color_u8!(255, 241, 164, 255),
        );
    }

    fn draw_difficulty_row(&self) {
        let Some(rects) = self.layout.difficulty_rects else {
            return;
        };

        for (difficulty, rect) in BotDifficulty::ALL.into_iter().zip(rects.into_iter()) {
            let control = match difficulty {
                BotDifficulty::Easy => MatchSetupControl::DifficultyEasy,
                BotDifficulty::Normal => MatchSetupControl::DifficultyNormal,
                BotDifficulty::Hard => MatchSetupControl::DifficultyHard,
            };

            draw_difficulty_button(
                rect,
                difficulty.label(),
                self.config.difficulty == Some(difficulty),
                self.hovered_control == Some(control),
                self.pressed_control == Some(control),
            );
        }
    }

    fn draw_asset_button(&self, control: MatchSetupControl, texture: &Texture2D) {
        let slot = self
            .layout
            .rect_for_control(control)
            .expect("asset button should have a rect");
        let is_hovered = self.hovered_control == Some(control);
        let is_pressed = self.pressed_control == Some(control);
        crate::arcade_ui::draw_interactive_texture_button(texture, slot, is_hovered, is_pressed);
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());

        if current_screen_size != self.last_screen_size {
            self.layout = MatchSetupLayout::from_screen(
                self.config.mode,
                current_screen_size.x,
                current_screen_size.y,
            );
            self.last_screen_size = current_screen_size;
        }
    }
}

