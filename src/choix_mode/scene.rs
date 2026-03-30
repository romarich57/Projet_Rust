use super::{
    assets::ModeSelectionAssets, buttons::ModeSelectionButton,
    layout::ModeSelectionLayout,
};
use crate::app::SceneCommand;
use crate::arcade_ui::{draw_interactive_texture_button, draw_slot_texture};
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
        let is_hovered = self.hovered_button == Some(button);
        let is_pressed = self.pressed_button == Some(button);
        draw_interactive_texture_button(texture, slot, is_hovered, is_pressed);
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

fn draw_vs_text(center: Vec2, font_size: f32) {
    let label = "VS";
    let metrics = measure_text(label, None, font_size as u16, 1.0);
    let x = center.x - metrics.width * 0.5;
    let y = center.y;

    draw_text(
        label,
        x + 5.0,
        y + 6.0,
        font_size,
        Color::new(0.35, 0.12, 0.0, 0.85),
    );
    draw_text(
        label,
        x + 2.0,
        y + 2.0,
        font_size,
        Color::new(1.0, 0.74, 0.15, 0.62),
    );
    draw_text(label, x, y, font_size, Color::new(1.0, 0.96, 0.80, 1.0));
}
