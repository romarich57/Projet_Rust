use crate::app::SceneCommand;
use crate::arcade_ui::{draw_cover_texture, draw_panel, draw_shadowed_centered_text, draw_slot_texture, fit_contain};
use crate::leaderboard::assets::LeaderboardAssets;
use crate::leaderboard::data::{LeaderboardData, MatchHistoryMode};
use crate::leaderboard::layout::{LeaderboardLayout, LeaderboardRowLayout};
use crate::physics::scale_y;
use macroquad::prelude::*;

const PANEL_LINE: Color = Color::new(0.45, 0.77, 1.0, 0.18);
const GOLD_TEXT: Color = Color::new(1.0, 0.9, 0.55, 0.98);
const EMPTY_TEXT: Color = Color::new(0.84, 0.91, 1.0, 0.95);

pub(crate) struct ScoreboardScene {
    background: Texture2D,
    victory_banner: Texture2D,
    defeat_banner: Texture2D,
    history_banner: Texture2D,
    return_button: Texture2D,
    mode_header: Texture2D,
    score_header: Texture2D,
    solo_badge: Texture2D,
    one_vs_one_badge: Texture2D,
    data: LeaderboardData,
    layout: LeaderboardLayout,
    hovered_return: bool,
    pressed_return: bool,
    last_screen_size: Vec2,
}

impl ScoreboardScene {
    pub(crate) fn new(assets: &LeaderboardAssets, data: LeaderboardData) -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        Self {
            background: assets.background.clone(),
            victory_banner: assets.victory_banner.clone(),
            defeat_banner: assets.defeat_banner.clone(),
            history_banner: assets.history_banner.clone(),
            return_button: assets.return_button.clone(),
            mode_header: assets.mode_header.clone(),
            score_header: assets.score_header.clone(),
            solo_badge: assets.solo_badge.clone(),
            one_vs_one_badge: assets.one_vs_one_badge.clone(),
            data,
            layout: LeaderboardLayout::from_screen(screen_size.x, screen_size.y),
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
        self.hovered_return = self.layout.return_button_slot.contains(mouse);

        if is_mouse_button_pressed(MouseButton::Left) && self.hovered_return {
            self.pressed_return = true;
        }

        if is_mouse_button_released(MouseButton::Left) {
            let should_return = self.pressed_return && self.hovered_return;
            self.pressed_return = false;
            if should_return {
                return SceneCommand::BackToMenu;
            }
        }

        if !is_mouse_button_down(MouseButton::Left) {
            self.pressed_return = false;
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
            Color::new(0.0, 0.02, 0.07, 0.24),
        );

        draw_slot_texture(&self.victory_banner, self.layout.victory_slot, 1.0);
        draw_slot_texture(&self.defeat_banner, self.layout.defeat_slot, 1.0);
        draw_slot_texture(&self.history_banner, self.layout.history_slot, 1.0);
        draw_panel(self.layout.panel_rect);
        draw_slot_texture(&self.mode_header, self.layout.mode_header_slot, 1.0);
        draw_slot_texture(&self.score_header, self.layout.score_header_slot, 1.0);
        self.draw_stats();
        self.draw_history_rows();
        self.draw_return_button();
    }

    fn draw_stats(&self) {
        draw_shadowed_centered_text(
            &self.data.solo_bot_wins.to_string(),
            self.layout.victory_value_pos.x,
            self.layout.victory_value_pos.y,
            66.0 * scale_y(),
            GOLD_TEXT,
        );
        draw_shadowed_centered_text(
            &self.data.solo_bot_losses.to_string(),
            self.layout.defeat_value_pos.x,
            self.layout.defeat_value_pos.y,
            66.0 * scale_y(),
            GOLD_TEXT,
        );
    }

    fn draw_history_rows(&self) {
        let rows = self.layout.visible_rows();
        if self.data.matches.is_empty() {
            draw_shadowed_centered_text(
                "AUCUN MATCH ENREGISTRE",
                self.layout.empty_state_center.x,
                self.layout.empty_state_center.y,
                28.0 * scale_y(),
                EMPTY_TEXT,
            );
            return;
        }

        for (layout, entry) in rows.iter().zip(self.data.matches.iter()) {
            draw_history_row(*layout, entry.mode, self.badge_for_mode(entry.mode));
            draw_shadowed_centered_text(
                &format!("{} - {}", entry.left_score, entry.right_score),
                layout.score_center_x,
                layout.score_baseline_y,
                34.0 * scale_y(),
                GOLD_TEXT,
            );
        }
    }

    fn draw_return_button(&self) {
        let scale = if self.pressed_return {
            0.98
        } else if self.hovered_return {
            1.03
        } else {
            1.0
        };
        draw_slot_texture(&self.return_button, self.layout.return_button_slot, scale);
    }

    fn badge_for_mode(&self, mode: MatchHistoryMode) -> &Texture2D {
        match mode {
            MatchHistoryMode::Solo => &self.solo_badge,
            MatchHistoryMode::OneVsOne => &self.one_vs_one_badge,
        }
    }

    fn refresh_layout_if_needed(&mut self) {
        let current_screen_size = vec2(screen_width(), screen_height());
        if current_screen_size != self.last_screen_size {
            self.layout =
                LeaderboardLayout::from_screen(current_screen_size.x, current_screen_size.y);
            self.last_screen_size = current_screen_size;
        }
    }
}

fn draw_history_row(layout: LeaderboardRowLayout, mode: MatchHistoryMode, badge: &Texture2D) {
    draw_line(
        layout.rect.x,
        layout.rect.bottom(),
        layout.rect.right(),
        layout.rect.bottom(),
        1.0,
        PANEL_LINE,
    );

    let badge_rect = fit_contain(layout.mode_slot, badge.width(), badge.height());
    draw_texture_ex(
        badge,
        badge_rect.x,
        badge_rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(badge_rect.w, badge_rect.h)),
            ..Default::default()
        },
    );

    if matches!(mode, MatchHistoryMode::OneVsOne) {
        draw_line(
            badge_rect.x + 6.0,
            badge_rect.y + badge_rect.h - 4.0,
            badge_rect.right() - 8.0,
            badge_rect.y + 6.0,
            1.6,
            Color::new(1.0, 0.7, 0.25, 0.25),
        );
    }
}
