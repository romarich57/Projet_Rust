use crate::gameplay::{GameplayAssets, GameplaySession, MatchConfig, MatchMode};
use crate::match_setup::{MatchSetupAssets, MatchSetupScene};
use crate::menu::{MenuAssets, MenuScene};
use crate::mode_selection::{ModeSelectionAssets, ModeSelectionScene};
use crate::scoreboard::ScoreboardScene;
use macroquad::prelude::*;

pub(crate) enum Scene {
    Menu(MenuScene),
    ModeSelection(ModeSelectionScene),
    MatchSetup(MatchSetupScene),
    Playing(GameplaySession),
    Scoreboard(ScoreboardScene),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SceneCommand {
    OpenModeSelection,
    OpenMatchSetup(MatchMode),
    StartMatch(MatchConfig),
    OpenScoreboard,
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
    scene: Scene,
}

impl App {
    pub(crate) async fn new() -> Result<Self, String> {
        let gameplay_assets = GameplayAssets::load().await?;
        let menu_assets = MenuAssets::load().await?;
        let mode_selection_assets = ModeSelectionAssets::load().await?;
        let match_setup_assets = MatchSetupAssets::load().await?;

        Ok(Self {
            gameplay_assets,
            menu_assets,
            mode_selection_assets,
            match_setup_assets,
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
                ));
            }
            SceneCommand::OpenScoreboard => {
                self.scene = Scene::Scoreboard(ScoreboardScene::new(&self.menu_assets));
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
