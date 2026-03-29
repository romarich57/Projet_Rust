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
    result_recorded: bool,
    finished_redirect_seconds: Option<f32>,
}

const MATCH_FINISHED_REDIRECT_DURATION: f32 = 3.0;

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
            result_recorded: false,
            finished_redirect_seconds: None,
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
            ensure_finished_redirect_started(&mut self.finished_redirect_seconds);

            if let Some(command) = pending_match_result_command(
                self.match_config,
                &self.soccer_match,
                self.result_recorded,
            ) {
                self.result_recorded = true;
                return command;
            }

            return tick_finished_redirect(&mut self.finished_redirect_seconds, get_frame_time())
                .unwrap_or(SceneCommand::None);
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

        if matches!(self.soccer_match.state, GameState::Finished) {
            ensure_finished_redirect_started(&mut self.finished_redirect_seconds);

            if let Some(command) = pending_match_result_command(
                self.match_config,
                &self.soccer_match,
                self.result_recorded,
            ) {
                self.result_recorded = true;
                return command;
            }
        }

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
            self.finished_redirect_seconds,
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

const HEAD_SIZE: f32 = 78.0;
const HEAD_OFFSET_X: f32 = 8.0;
const HEAD_OFFSET_Y: f32 = -52.0;
const FOOT_TO_HEAD_HEIGHT_RATIO: f32 = 0.70;
const FOOT_TEXTURE_ASPECT_RATIO: f32 = 612.0 / 408.0;
const FOOT_GROUND_CONTACT_RATIO: f32 = 0.72;
const FOOT_PIVOT_FROM_BACK_X_RATIO: f32 = 0.18;
const FOOT_PIVOT_Y_RATIO: f32 = 0.58;
const FOOT_HITBOX_OFFSET_X_RATIO: f32 = 22.0 / 114.0;
const FOOT_HITBOX_OFFSET_Y_RATIO: f32 = 12.0 / 50.0;
const FOOT_HITBOX_WIDTH_RATIO: f32 = 65.0 / 114.0;
const FOOT_HITBOX_HEIGHT_RATIO: f32 = 30.0 / 50.0;
const HEAD_HITBOX_OFFSET_X_RATIO: f32 = 12.0 / 78.0;
const HEAD_HITBOX_OFFSET_Y_RATIO: f32 = 8.0 / 78.0;
const HEAD_HITBOX_WIDTH_RATIO: f32 = 54.0 / 78.0;
const HEAD_HITBOX_HEIGHT_RATIO: f32 = 62.0 / 78.0;

fn apply_match_player_tuning(player: &mut Player, arena: &ArenaGeometry) {
    let scale = arena.uniform_scale;
    let head_size = HEAD_SIZE * scale;
    let foot_height = head_size * FOOT_TO_HEAD_HEIGHT_RATIO;
    let foot_width = foot_height * FOOT_TEXTURE_ASPECT_RATIO;

    player.foot_width = foot_width;
    player.foot_height = foot_height;
    player.head_width = head_size;
    player.head_height = head_size;
    player.head_offset_x = HEAD_OFFSET_X * scale;
    player.head_offset_y = HEAD_OFFSET_Y * scale;
    player.set_foot_visual_anchor(
        FOOT_GROUND_CONTACT_RATIO,
        FOOT_PIVOT_FROM_BACK_X_RATIO,
        FOOT_PIVOT_Y_RATIO,
    );
    player.set_foot_hitbox(
        foot_width * FOOT_HITBOX_OFFSET_X_RATIO,
        foot_height * FOOT_HITBOX_OFFSET_Y_RATIO,
        foot_width * FOOT_HITBOX_WIDTH_RATIO,
        foot_height * FOOT_HITBOX_HEIGHT_RATIO,
    );
    player.set_head_hitbox(
        head_size * HEAD_HITBOX_OFFSET_X_RATIO,
        head_size * HEAD_HITBOX_OFFSET_Y_RATIO,
        head_size * HEAD_HITBOX_WIDTH_RATIO,
        head_size * HEAD_HITBOX_HEIGHT_RATIO,
    );
}

fn pending_match_result_command(
    match_config: MatchConfig,
    soccer_match: &Match,
    result_recorded: bool,
) -> Option<SceneCommand> {
    if result_recorded || !matches!(soccer_match.state, GameState::Finished) {
        return None;
    }

    let (left_score, right_score) = soccer_match.score();
    Some(SceneCommand::RecordMatchResult {
        mode: match_config.mode,
        left_score: normalize_score(left_score),
        right_score: normalize_score(right_score),
    })
}

fn normalize_score(score: i32) -> u8 {
    score.clamp(0, u8::MAX as i32) as u8
}

fn ensure_finished_redirect_started(finished_redirect_seconds: &mut Option<f32>) {
    if finished_redirect_seconds.is_none() {
        *finished_redirect_seconds = Some(MATCH_FINISHED_REDIRECT_DURATION);
    }
}

fn tick_finished_redirect(
    finished_redirect_seconds: &mut Option<f32>,
    delta_seconds: f32,
) -> Option<SceneCommand> {
    let remaining = finished_redirect_seconds.as_mut()?;
    *remaining = (*remaining - delta_seconds).max(0.0);

    if *remaining <= 0.0 {
        Some(SceneCommand::BackToMenu)
    } else {
        None
    }
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

    #[test]
    fn pending_match_result_command_returns_finished_score_once() {
        let config = MatchConfig::default_for_mode(MatchMode::Solo);
        let mut soccer_match = Match::new(0.0);
        soccer_match.score_p1 = 3;
        soccer_match.score_p2 = 1;
        soccer_match.state = GameState::Finished;

        let command = pending_match_result_command(config, &soccer_match, false);

        assert_eq!(
            command,
            Some(SceneCommand::RecordMatchResult {
                mode: MatchMode::Solo,
                left_score: 3,
                right_score: 1,
            })
        );
    }

    #[test]
    fn pending_match_result_command_returns_none_when_already_recorded() {
        let config = MatchConfig::default_for_mode(MatchMode::OneVsOne);
        let mut soccer_match = Match::new(0.0);
        soccer_match.score_p1 = 2;
        soccer_match.score_p2 = 2;
        soccer_match.state = GameState::Finished;

        assert_eq!(
            pending_match_result_command(config, &soccer_match, true),
            None
        );
    }

    #[test]
    fn finished_redirect_timer_starts_once() {
        let mut timer = None;

        ensure_finished_redirect_started(&mut timer);
        ensure_finished_redirect_started(&mut timer);

        assert_eq!(timer, Some(MATCH_FINISHED_REDIRECT_DURATION));
    }

    #[test]
    fn finished_redirect_waits_until_zero_before_back_to_menu() {
        let mut timer = Some(1.5);

        assert_eq!(tick_finished_redirect(&mut timer, 1.0), None);
        assert_eq!(timer, Some(0.5));
        assert_eq!(
            tick_finished_redirect(&mut timer, 0.5),
            Some(SceneCommand::BackToMenu)
        );
        assert_eq!(timer, Some(0.0));
    }
}
