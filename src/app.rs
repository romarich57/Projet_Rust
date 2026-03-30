use crate::gameplay::{GameplayAssets, GameplaySession, MatchConfig, MatchMode};
use crate::leaderboard::assets::LeaderboardAssets;
use crate::leaderboard::scene::ScoreboardScene;
use crate::leaderboard::storage::LeaderboardStore;
use crate::match_setup::{MatchSetupAssets, MatchSetupScene};
use crate::menu::{MenuAssets, MenuScene};
use crate::mode_selection::{ModeSelectionAssets, ModeSelectionScene};
use crate::settings::{
    SettingsAssets, SettingsData, SettingsFeedback, SettingsScene, SettingsStore,
};
use macroquad::prelude::*;

pub(crate) enum Scene {
    Menu(MenuScene),
    ModeSelection(ModeSelectionScene),
    MatchSetup(MatchSetupScene),
    Playing(GameplaySession),
    Scoreboard(ScoreboardScene),
    Settings(SettingsScene),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SceneCommand {
    OpenModeSelection,
    OpenMatchSetup(MatchMode),
    StartMatch(MatchConfig),
    OpenScoreboard,
    OpenSettings,
    SaveSettings(SettingsData),
    RecordMatchResult {
        mode: MatchMode,
        left_score: u8,
        right_score: u8,
    },
    BackToMenu,
    BackToModeSelection,
    Quit,
    None,
}

pub(crate) struct App {
    gameplay_assets: GameplayAssets,
    menu_assets: MenuAssets,
    mode_selection_assets: ModeSelectionAssets,
    match_setup_assets: MatchSetupAssets,
    leaderboard_assets: LeaderboardAssets,
    leaderboard_store: LeaderboardStore,
    settings_assets: SettingsAssets,
    settings_store: SettingsStore,
    scene: Scene,
}

impl App {
    pub(crate) async fn new() -> Result<Self, String> {
        let gameplay_assets = GameplayAssets::load().await?;
        let menu_assets = MenuAssets::load().await?;
        let mode_selection_assets = ModeSelectionAssets::load().await?;
        let match_setup_assets = MatchSetupAssets::load().await?;
        let leaderboard_assets = LeaderboardAssets::load().await?;
        let leaderboard_store = LeaderboardStore::load_or_default("save/leaderboard.json")?;
        let settings_assets = SettingsAssets::load().await?;
        let settings_store = SettingsStore::load_or_default("save/settings.json")?;

        Ok(Self {
            gameplay_assets,
            menu_assets,
            mode_selection_assets,
            match_setup_assets,
            leaderboard_assets,
            leaderboard_store,
            settings_assets,
            settings_store,
            scene: Scene::Menu(MenuScene::new()),
        })
    }

    pub(crate) fn update(&mut self) {
        let command = match &mut self.scene {
            Scene::Menu(scene) => scene.update(),
            Scene::ModeSelection(scene) => scene.update(),
            Scene::MatchSetup(scene) => scene.update(),
            Scene::Playing(session) => session.update(),
            Scene::Scoreboard(scene) => scene.update(),
            Scene::Settings(scene) => scene.update(),
        };

        self.apply_command(command);
    }

    pub(crate) fn draw(&self) {
        clear_background(BLACK);

        match &self.scene {
            Scene::Menu(scene) => scene.draw(&self.menu_assets),
            Scene::ModeSelection(scene) => scene.draw(&self.mode_selection_assets),
            Scene::MatchSetup(scene) => scene.draw(&self.match_setup_assets),
            Scene::Playing(session) => session.draw(),
            Scene::Scoreboard(scene) => scene.draw(),
            Scene::Settings(scene) => scene.draw(),
        }
    }

    fn apply_command(&mut self, command: SceneCommand) {
        match command {
            SceneCommand::OpenModeSelection => {
                self.scene = Scene::ModeSelection(ModeSelectionScene::new());
            }
            SceneCommand::OpenMatchSetup(mode) => {
                self.scene = Scene::MatchSetup(MatchSetupScene::new(mode));
            }
            SceneCommand::StartMatch(config) => {
                self.scene = Scene::Playing(GameplaySession::new_for_config(
                    &self.gameplay_assets,
                    config,
                    self.settings_store.snapshot().controls,
                ));
            }
            SceneCommand::OpenScoreboard => {
                self.scene = Scene::Scoreboard(ScoreboardScene::new(
                    &self.leaderboard_assets,
                    self.leaderboard_store.snapshot(),
                ));
            }
            SceneCommand::OpenSettings => {
                self.scene = Scene::Settings(SettingsScene::new(
                    &self.settings_assets,
                    self.settings_store.snapshot(),
                ));
            }
            SceneCommand::SaveSettings(draft) => {
                let saved_snapshot = self.settings_store.snapshot();
                let feedback = match self.settings_store.replace_and_persist(draft) {
                    Ok(()) => {
                        self.scene = Scene::Settings(SettingsScene::from_state(
                            &self.settings_assets,
                            draft,
                            draft,
                            Some(SettingsFeedback::success("Touches sauvegardees")),
                        ));
                        return;
                    }
                    Err(err) => SettingsFeedback::error(format!("Echec de sauvegarde: {err}")),
                };

                self.scene = Scene::Settings(SettingsScene::from_state(
                    &self.settings_assets,
                    saved_snapshot,
                    draft,
                    Some(feedback),
                ));
            }
            SceneCommand::RecordMatchResult {
                mode,
                left_score,
                right_score,
            } => {
                if let Err(err) = self
                    .leaderboard_store
                    .record_match(mode, left_score, right_score)
                {
                    eprintln!("failed to persist leaderboard data: {err}");
                }
            }
            SceneCommand::BackToMenu => {
                self.scene = Scene::Menu(MenuScene::new());
            }
            SceneCommand::BackToModeSelection => {
                self.scene = Scene::ModeSelection(ModeSelectionScene::new());
            }
            SceneCommand::Quit => {
                macroquad::miniquad::window::quit();
            }
            SceneCommand::None => {}
        }
    }
}
