use crate::app::SceneCommand;
use crate::arcade_ui::{load_linear_texture, load_svg_texture};
use crate::game::{GameState, Match};
use crate::ia;
use crate::input;
use crate::match_arena::ArenaGeometry;
use crate::match_hud::{
    draw_hud, HudAction, HudAssets, HudInteractionState, HudLayout, HudVisualState,
};
use crate::models::ball::Ball;
use crate::models::player::{ControlType, Player};
use crate::physics;
use crate::physics::player_physics;
use crate::render;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MatchMode {
    Solo,
    OneVsOne,
}

impl MatchMode {
    pub(crate) fn shows_difficulty(self) -> bool {
        matches!(self, Self::Solo)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlayerProfile {
    Fiorio,
    Dejonckere,
}

impl PlayerProfile {
    pub(crate) fn display_name(self) -> &'static str {
        match self {
            Self::Fiorio => "FIORIO",
            Self::Dejonckere => "DEJONCKERE",
        }
    }

    pub(crate) fn next(self) -> Self {
        match self {
            Self::Fiorio => Self::Dejonckere,
            Self::Dejonckere => Self::Fiorio,
        }
    }

    pub(crate) fn previous(self) -> Self {
        self.next()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MatchLength {
    OneMinute,
    TwoMinutes,
    ThreeMinutes,
}

impl MatchLength {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::OneMinute => "1 MIN",
            Self::TwoMinutes => "2 MIN",
            Self::ThreeMinutes => "3 MIN",
        }
    }

    pub(crate) fn seconds(self) -> f32 {
        match self {
            Self::OneMinute => 60.0,
            Self::TwoMinutes => 120.0,
            Self::ThreeMinutes => 180.0,
        }
    }

    pub(crate) fn next(self) -> Self {
        match self {
            Self::OneMinute => Self::TwoMinutes,
            Self::TwoMinutes => Self::ThreeMinutes,
            Self::ThreeMinutes => Self::OneMinute,
        }
    }

    pub(crate) fn previous(self) -> Self {
        match self {
            Self::OneMinute => Self::ThreeMinutes,
            Self::TwoMinutes => Self::OneMinute,
            Self::ThreeMinutes => Self::TwoMinutes,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BotDifficulty {
    Easy,
    Normal,
    Hard,
}

impl BotDifficulty {
    pub(crate) const ALL: [Self; 3] = [Self::Easy, Self::Normal, Self::Hard];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Easy => "FACILE",
            Self::Normal => "NORMAL",
            Self::Hard => "DIFFICILE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MatchConfig {
    pub(crate) mode: MatchMode,
    pub(crate) left_player: PlayerProfile,
    pub(crate) right_player: PlayerProfile,
    pub(crate) length: MatchLength,
    pub(crate) difficulty: Option<BotDifficulty>,
}

impl MatchConfig {
    pub(crate) fn default_for_mode(mode: MatchMode) -> Self {
        Self {
            mode,
            left_player: PlayerProfile::Fiorio,
            right_player: PlayerProfile::Dejonckere,
            length: MatchLength::TwoMinutes,
            difficulty: if mode.shows_difficulty() {
                Some(BotDifficulty::Normal)
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TeamSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameplayOverlayState {
    Running,
    Paused,
    ConfirmQuit,
}

pub(crate) struct GameplayAssets {
    ball_texture: Texture2D,
    terrain_texture: Texture2D,
    goal_texture: Texture2D,
    foot_texture: Texture2D,
    left_fiorio_head: Texture2D,
    left_dejonckere_head: Texture2D,
    right_fiorio_head: Texture2D,
    right_dejonckere_head: Texture2D,
    pause_icon: Texture2D,
    continue_icon: Texture2D,
    quit_icon: Texture2D,
}

impl GameplayAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            ball_texture: load_linear_texture("src/assets/ballon/ballon.png").await?,
            terrain_texture: load_linear_texture("src/assets/stade/Terrain.png").await?,
            goal_texture: load_svg_texture("src/assets/cage/cage.svg", uvec2(256, 384)).await?,
            foot_texture: load_linear_texture("src/assets/joueur/pied.png").await?,
            left_fiorio_head: load_linear_texture("src/assets/joueur/joueur_gauche/fiorio1.png")
                .await?,
            left_dejonckere_head: load_linear_texture(
                "src/assets/joueur/joueur_gauche/dejonckere1.png",
            )
            .await?,
            right_fiorio_head: load_linear_texture("src/assets/joueur/joueur_droite/fiorio.png")
                .await?,
            right_dejonckere_head: load_linear_texture(
                "src/assets/joueur/joueur_droite/dejonckere.png",
            )
            .await?,
            pause_icon: load_svg_texture("src/assets/navbar/pause.svg", uvec2(128, 128)).await?,
            continue_icon: load_svg_texture("src/assets/navbar/continuer.svg", uvec2(128, 128))
                .await?,
            quit_icon: load_svg_texture("src/assets/navbar/quitter.svg", uvec2(128, 128)).await?,
        })
    }
}

pub(crate) struct GameplaySession {
    terrain_texture: Texture2D,
    goal_texture: Texture2D,
    hud_assets: HudAssets,
    ball: Ball,
    players: Vec<Player>,
    arena: ArenaGeometry,
    hud_layout: HudLayout,
    hud_interaction: HudInteractionState,
    last_screen_size: Vec2,
    debug_hitbox: bool,
    overlay_state: GameplayOverlayState,
    pub(crate) match_config: MatchConfig,
    soccer_match: Match,
}

impl GameplaySession {
    pub(crate) fn new_for_config(assets: &GameplayAssets, config: MatchConfig) -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        let arena = ArenaGeometry::from_screen(screen_size.x, screen_size.y);
        let hud_layout = HudLayout::from_screen(screen_size.x, screen_size.y);
        let hud_assets = HudAssets::new(
            assets.pause_icon.clone(),
            assets.continue_icon.clone(),
            assets.quit_icon.clone(),
        );

        let ball_visual_radius = 26.0 * arena.uniform_scale;
        let mut ball = Ball::new(0.0, 0.0, ball_visual_radius, assets.ball_texture.clone());
        ball.set_circle_hitbox(0.0, 0.0, ball.visual_radius() * 0.72);
        let spawn = arena.ball_spawn_position(ball.hitbox.radius, ball.hitbox.offset_y);
        ball.x = spawn.x;
        ball.y = spawn.y;

        let players = build_players_for_config(assets, config, &arena);

        Self {
            terrain_texture: assets.terrain_texture.clone(),
            goal_texture: assets.goal_texture.clone(),
            hud_assets,
            ball,
            players,
            arena,
            hud_layout,
            hud_interaction: HudInteractionState::default(),
            last_screen_size: screen_size,
            debug_hitbox: false,
            overlay_state: GameplayOverlayState::Running,
            match_config: config,
            soccer_match: Match::new(config.length.seconds()),
        }
    }

    pub(crate) fn update(&mut self) -> SceneCommand {
        self.refresh_layout_if_needed();

        if is_key_pressed(KeyCode::Y) {
            self.debug_hitbox = !self.debug_hitbox;
        }

        match self
            .hud_interaction
            .update(self.hud_layout, self.hud_visual_state())
        {
            HudAction::TogglePause => match self.overlay_state {
                GameplayOverlayState::Running if self.soccer_match.state != GameState::Finished => {
                    self.overlay_state = GameplayOverlayState::Paused;
                }
                GameplayOverlayState::Paused => {
                    self.overlay_state = GameplayOverlayState::Running;
                }
                GameplayOverlayState::ConfirmQuit => {}
                GameplayOverlayState::Running => {}
            },
            HudAction::OpenQuitConfirmation => {
                self.overlay_state = GameplayOverlayState::ConfirmQuit;
            }
            HudAction::ConfirmQuit => return SceneCommand::BackToMenu,
            HudAction::CancelQuit => {
                self.overlay_state = GameplayOverlayState::Paused;
            }
            HudAction::None => {}
        }

        if self.overlay_state != GameplayOverlayState::Running {
            return SceneCommand::None;
        }

        if matches!(self.soccer_match.state, GameState::Finished) {
            return SceneCommand::None;
        }

        let _selected_difficulty = self.match_config.difficulty;

        for player in &mut self.players {
            match player.control_type {
                ControlType::IA => ia::handle_ai(player, &self.ball, &self.arena),
                _ => input::handle_keyboard(player),
            }
            input::update_animations(player);
            player_physics::apply_physics(player, &self.arena);
        }

        if self.players.len() >= 2 {
            let (left, right) = self.players.split_at_mut(1);
            physics::collision::apply_player_player_collision(&mut left[0], &mut right[0]);
        }

        for player in &self.players {
            physics::collision::apply_player_ball_collision(player, &mut self.ball);
        }

        physics::ball_physics::apply_ball_physics(&mut self.ball, &self.arena);
        self.soccer_match.update(
            &mut self.ball,
            &mut self.players,
            &self.arena,
            get_frame_time(),
        );

        SceneCommand::None
    }

    pub(crate) fn draw(&self) {
        render::draw_all(
            &self.players,
            &self.terrain_texture,
            &self.goal_texture,
            &self.arena,
            &self.ball,
            self.debug_hitbox,
            &self.soccer_match,
        );

        let (score_left, score_right) = self.soccer_match.score();
        draw_hud(
            self.hud_layout,
            &self.hud_interaction,
            &self.hud_assets,
            self.hud_visual_state(),
            self.soccer_match.remaining_time_seconds(),
            score_left,
            score_right,
            self.state_label(),
        );
    }

    fn hud_visual_state(&self) -> HudVisualState {
        match self.overlay_state {
            GameplayOverlayState::ConfirmQuit => HudVisualState::ConfirmQuit,
            GameplayOverlayState::Paused => HudVisualState::Paused,
            GameplayOverlayState::Running => {
                if self.soccer_match.state == GameState::Finished {
                    HudVisualState::Finished
                } else {
                    HudVisualState::Running
                }
            }
        }
    }

    fn state_label(&self) -> &'static str {
        match self.overlay_state {
            GameplayOverlayState::Paused | GameplayOverlayState::ConfirmQuit => "Pause",
            GameplayOverlayState::Running => match self.soccer_match.state {
                GameState::Playing => "En jeu",
                GameState::GoalScored { .. } => "But",
                GameState::Finished => "Termine",
            },
        }
    }

    fn refresh_layout_if_needed(&mut self) {
        let screen_size = vec2(screen_width(), screen_height());

        if screen_size != self.last_screen_size {
            self.arena = ArenaGeometry::from_screen(screen_size.x, screen_size.y);
            self.hud_layout = HudLayout::from_screen(screen_size.x, screen_size.y);

            for player in &mut self.players {
                apply_match_player_tuning(player, &self.arena);
                player.x = self
                    .arena
                    .player_spawn_x(player.side, player.collision_width());
                player.y = player.y_at_ground(self.arena.ground_y);
            }

            let spawn = self
                .arena
                .ball_spawn_position(self.ball.hitbox.radius, self.ball.hitbox.offset_y);
            self.ball.x = self.ball.x.clamp(0.0, screen_size.x);
            self.ball.y = self.ball.y.clamp(self.arena.hud_height, screen_size.y);
            if !self.ball.x.is_finite() || !self.ball.y.is_finite() {
                self.ball.x = spawn.x;
                self.ball.y = spawn.y;
            }

            self.last_screen_size = screen_size;
        }
    }
}

fn build_players_for_config(
    assets: &GameplayAssets,
    config: MatchConfig,
    arena: &ArenaGeometry,
) -> Vec<Player> {
    let (left_control, right_control) = control_types_for_mode(config.mode);

    let mut left_player = Player::new(
        0.0,
        0.0,
        head_texture_for_side(assets, config.left_player, TeamSide::Left),
        assets.foot_texture.clone(),
        left_control,
        -1,
    );
    apply_match_player_tuning(&mut left_player, arena);
    left_player.x = arena.player_spawn_x(left_player.side, left_player.collision_width());
    left_player.y = left_player.y_at_ground(arena.ground_y);

    let mut right_player = Player::new(
        0.0,
        0.0,
        head_texture_for_side(assets, config.right_player, TeamSide::Right),
        assets.foot_texture.clone(),
        right_control,
        1,
    );
    apply_match_player_tuning(&mut right_player, arena);
    right_player.x = arena.player_spawn_x(right_player.side, right_player.collision_width());
    right_player.y = right_player.y_at_ground(arena.ground_y);

    vec![left_player, right_player]
}

fn control_types_for_mode(mode: MatchMode) -> (ControlType, ControlType) {
    match mode {
        MatchMode::Solo => (ControlType::Player1, ControlType::IA),
        MatchMode::OneVsOne => (ControlType::Player1, ControlType::Player2),
    }
}

fn head_texture_for_side(
    assets: &GameplayAssets,
    profile: PlayerProfile,
    side: TeamSide,
) -> Texture2D {
    match (side, profile) {
        (TeamSide::Left, PlayerProfile::Fiorio) => assets.left_fiorio_head.clone(),
        (TeamSide::Left, PlayerProfile::Dejonckere) => assets.left_dejonckere_head.clone(),
        (TeamSide::Right, PlayerProfile::Fiorio) => assets.right_fiorio_head.clone(),
        (TeamSide::Right, PlayerProfile::Dejonckere) => assets.right_dejonckere_head.clone(),
    }
}

fn apply_match_player_tuning(player: &mut Player, arena: &ArenaGeometry) {
    let scale = arena.uniform_scale;

    player.foot_width = 176.0 * scale;
    player.foot_height = 78.0 * scale;
    player.head_width = 118.0 * scale;
    player.head_height = 118.0 * scale;
    player.head_offset_x = 34.0 * scale;
    player.head_offset_y = -92.0 * scale;
    player.set_foot_hitbox(34.0 * scale, 20.0 * scale, 98.0 * scale, 44.0 * scale);
    player.set_head_hitbox(17.0 * scale, 10.0 * scale, 84.0 * scale, 92.0 * scale);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_builds_human_and_ai_controls() {
        assert_eq!(
            control_types_for_mode(MatchMode::Solo),
            (ControlType::Player1, ControlType::IA)
        );
    }

    #[test]
    fn one_vs_one_builds_two_human_controls() {
        assert_eq!(
            control_types_for_mode(MatchMode::OneVsOne),
            (ControlType::Player1, ControlType::Player2)
        );
    }
}
