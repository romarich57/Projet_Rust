use crate::app::SceneCommand;
use crate::arcade_ui::fit_contain;
use crate::menu::MenuAssets;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

pub(crate) struct ScoreboardScene {
    background: Texture2D,
    logo: Texture2D,
    layout: ScoreboardLayout,
    hovered_return: bool,
    pressed_return: bool,
    last_screen_size: Vec2,
}

impl ScoreboardScene {
    pub(crate) fn new(menu_assets: &MenuAssets) -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        Self {
            background: menu_assets.background.clone(),
            logo: menu_assets.logo.clone(),
            layout: ScoreboardLayout::from_screen(screen_size.x, screen_size.y),
            hovered_return: false,
            pressed_return: false,
            last_screen_size: screen_size,
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        if is_key_pressed(KeyCode::Escape) {
            return SceneCommand::BackToMenu;
        }

        let mouse = vec2(mouse_position().0, mouse_position().1);
        self.hovered_return = self.layout.return_button_rect.contains(mouse);

        if is_mouse_button_pressed(MouseButton::Left) && self.hovered_return {
            self.pressed_return = true;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let should_return = self.pressed_return && self.hovered_return;
            self.pressed_return = false;
            return if should_return {
                SceneCommand::BackToMenu
            } else {
                SceneCommand::None
            };
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_return = false;
        }

        SceneCommand::None
    }

    pub(crate) fn draw(&self) {
        draw_texture_ex(
            &self.background,
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
            Color::new(0.0, 0.0, 0.0, 0.48),
        );

        let logo_rect = fit_contain(self.layout.logo_slot, self.logo.width(), self.logo.height());
        draw_texture_ex(
            &self.logo,
            logo_rect.x,
            logo_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(logo_rect.w, logo_rect.h)),
                ..Default::default()
            },
        );

        draw_rectangle(
            self.layout.panel_rect.x + 6.0,
            self.layout.panel_rect.y + 8.0,
            self.layout.panel_rect.w,
            self.layout.panel_rect.h,
            Color::new(0.0, 0.0, 0.0, 0.35),
        );
        draw_rectangle(
            self.layout.panel_rect.x,
            self.layout.panel_rect.y,
            self.layout.panel_rect.w,
            self.layout.panel_rect.h,
            Color::new(0.06, 0.10, 0.18, 0.92),
        );
        draw_rectangle_lines(
            self.layout.panel_rect.x,
            self.layout.panel_rect.y,
            self.layout.panel_rect.w,
            self.layout.panel_rect.h,
            3.0,
            Color::new(0.36, 0.92, 1.0, 0.95),
        );

        draw_centered_text(
            "SCOREBOARD",
            self.layout.panel_rect.x + self.layout.panel_rect.w * 0.5,
            self.layout.panel_rect.y + 72.0,
            42.0,
            YELLOW,
        );
        draw_centered_text(
            "Fonctionnalite bientot disponible",
            self.layout.panel_rect.x + self.layout.panel_rect.w * 0.5,
            self.layout.panel_rect.y + 132.0,
            28.0,
            WHITE,
        );
        draw_centered_text(
            "Appuyez sur Echap ou cliquez sur Retour",
            self.layout.panel_rect.x + self.layout.panel_rect.w * 0.5,
            self.layout.panel_rect.y + 172.0,
            22.0,
            Color::new(0.80, 0.90, 1.0, 0.95),
        );

        let button_color = match (self.hovered_return, self.pressed_return) {
            (_, true) => Color::new(0.14, 0.40, 0.66, 1.0),
            (true, false) => Color::new(0.18, 0.50, 0.78, 1.0),
            _ => Color::new(0.10, 0.30, 0.54, 1.0),
        };

        draw_rectangle(
            self.layout.return_button_rect.x,
            self.layout.return_button_rect.y,
            self.layout.return_button_rect.w,
            self.layout.return_button_rect.h,
            button_color,
        );
        draw_rectangle_lines(
            self.layout.return_button_rect.x,
            self.layout.return_button_rect.y,
            self.layout.return_button_rect.w,
            self.layout.return_button_rect.h,
            3.0,
            Color::new(0.36, 0.92, 1.0, 0.95),
        );
        draw_centered_text(
            "Retour",
            self.layout.return_button_rect.x + self.layout.return_button_rect.w * 0.5,
            self.layout.return_button_rect.y + self.layout.return_button_rect.h * 0.62,
            28.0,
            WHITE,
        );
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());

        if current_screen_size != self.last_screen_size {
            self.layout =
                ScoreboardLayout::from_screen(current_screen_size.x, current_screen_size.y);
            self.last_screen_size = current_screen_size;
        }
    }
}

#[derive(Clone, Copy)]
struct ScoreboardLayout {
    logo_slot: Rect,
    panel_rect: Rect,
    return_button_rect: Rect,
}

impl ScoreboardLayout {
    fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;

        Self {
            logo_slot: Rect::new(
                385.0 * scale_x,
                32.0 * scale_y,
                230.0 * scale_x,
                105.0 * scale_y,
            ),
            panel_rect: Rect::new(
                220.0 * scale_x,
                156.0 * scale_y,
                560.0 * scale_x,
                270.0 * scale_y,
            ),
            return_button_rect: Rect::new(
                410.0 * scale_x,
                360.0 * scale_y,
                180.0 * scale_x,
                54.0 * scale_y,
            ),
        }
    }
}

fn draw_centered_text(text: &str, center_x: f32, baseline_y: f32, font_size: f32, color: Color) {
    let metrics = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        center_x - metrics.width * 0.5,
        baseline_y,
        font_size,
        color,
    );
}
