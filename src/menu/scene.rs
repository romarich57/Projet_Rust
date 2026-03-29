use super::{assets::MenuAssets, buttons::MenuButton, layout::MenuLayout};
use crate::app::SceneCommand;
use crate::arcade_ui::draw_slot_texture;
use macroquad::prelude::*;

pub(crate) struct MenuScene {
    layout: MenuLayout,
    hovered_button: Option<MenuButton>,
    pressed_button: Option<MenuButton>,
    last_screen_size: Vec2,
}

impl MenuScene {
    pub(crate) fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        Self {
            layout: MenuLayout::from_screen(screen_size.x, screen_size.y),
            hovered_button: None,
            pressed_button: None,
            last_screen_size: screen_size,
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered_button = MenuButton::ALL
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

    pub(crate) fn draw(&self, assets: &MenuAssets) {
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

        draw_slot_texture(&assets.messi, self.layout.messi_slot, 1.0);
        draw_slot_texture(&assets.ronaldo, self.layout.ronaldo_slot, 1.0);
        draw_slot_texture(&assets.logo, self.layout.logo_slot, 1.0);

        self.draw_button(MenuButton::Play, &assets.play_button);
        self.draw_button(MenuButton::Scoreboard, &assets.scoreboard_button);
        self.draw_button(MenuButton::Quit, &assets.quit_button);
    }

    fn draw_button(&self, button: MenuButton, texture: &Texture2D) {
        let scale = match (
            self.hovered_button == Some(button),
            self.pressed_button == Some(button),
        ) {
            (_, true) => 0.98,
            (true, false) => 1.03,
            _ => 1.0,
        };

        draw_slot_texture(texture, self.layout.button_rect(button), scale);
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());

        if current_screen_size != self.last_screen_size {
            self.layout = MenuLayout::from_screen(current_screen_size.x, current_screen_size.y);
            self.last_screen_size = current_screen_size;
        }
    }
}
