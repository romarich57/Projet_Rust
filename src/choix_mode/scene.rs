use super::{
    assets::ModeSelectionAssets, buttons::ModeSelectionButton, draw::draw_vs_text,
    layout::ModeSelectionLayout,
};
use crate::app::SceneCommand;
use crate::arcade_ui::{draw_slot_texture, fit_contain, scale_rect_from_center};
use macroquad::prelude::*;

pub(crate) struct ModeSelectionScene {
    layout: ModeSelectionLayout,
    hovered_button: Option<ModeSelectionButton>,
    pressed_button: Option<ModeSelectionButton>,
    last_screen_size: Vec2,
}

impl ModeSelectionScene {
    pub(crate) fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        Self {
            layout: ModeSelectionLayout::from_screen(screen_size.x, screen_size.y),
            hovered_button: None,
            pressed_button: None,
            last_screen_size: screen_size,
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        if is_key_pressed(KeyCode::Escape) {
            return SceneCommand::BackToMenu;
        }

        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered_button = ModeSelectionButton::ALL
            .into_iter()
            .find(|button| self.layout.button_rect(*button).contains(mouse));

        if is_mouse_button_pressed(MouseButton::Left) {
            self.pressed_button = self.hovered_button;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let command = match (self.pressed_button, self.hovered_button) {
                (Some(pressed), Some(hovered)) if pressed == hovered => hovered.command(),
                _ => SceneCommand::None,
            };
            self.pressed_button = None;
            return command;
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_button = None;
        }

        SceneCommand::None
    }

    pub(crate) fn draw(&self, assets: &ModeSelectionAssets) {
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
            Color::new(0.0, 0.0, 0.0, 0.42),
        );

        draw_slot_texture(&assets.title, self.layout.title_slot, 1.0);

        self.draw_button(ModeSelectionButton::Solo, &assets.solo_button);
        self.draw_button(ModeSelectionButton::OneVsOne, &assets.one_vs_one_button);

        draw_vs_text(self.layout.vs_center, self.layout.vs_font_size);

        draw_slot_texture(&assets.materazzi, self.layout.materazzi_slot, 1.0);
        draw_slot_texture(&assets.zidane, self.layout.zidane_slot, 1.0);

        self.draw_button(ModeSelectionButton::Back, &assets.back_button);
    }

    fn draw_button(&self, button: ModeSelectionButton, texture: &Texture2D) {
        let slot = self.layout.button_rect(button);
        let base_rect = fit_contain(slot, texture.width(), texture.height());
        let is_hovered = self.hovered_button == Some(button);
        let is_pressed = self.pressed_button == Some(button);

        if is_hovered || is_pressed {
            let glow_rect = scale_rect_from_center(base_rect, 1.07);
            draw_texture_ex(
                texture,
                glow_rect.x,
                glow_rect.y,
                Color::new(1.0, 1.0, 1.0, 0.30),
                DrawTextureParams {
                    dest_size: Some(vec2(glow_rect.w, glow_rect.h)),
                    ..Default::default()
                },
            );
        }

        let scale = match (is_hovered, is_pressed) {
            (_, true) => 0.98,
            (true, false) => 1.03,
            _ => 1.0,
        };
        draw_slot_texture(texture, slot, scale);
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());

        if current_screen_size != self.last_screen_size {
            self.layout =
                ModeSelectionLayout::from_screen(current_screen_size.x, current_screen_size.y);
            self.last_screen_size = current_screen_size;
        }
    }
}
