use crate::app::SceneCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MenuButton {
    Play,
    Scoreboard,
    Quit,
    Settings,
}

impl MenuButton {
    pub(super) const ALL: [Self; 4] = [Self::Play, Self::Scoreboard, Self::Quit, Self::Settings];

    pub(super) fn command(self) -> SceneCommand {
        match self {
            Self::Play => SceneCommand::OpenModeSelection,
            Self::Scoreboard => SceneCommand::OpenScoreboard,
            Self::Quit => SceneCommand::Quit,
            Self::Settings => SceneCommand::OpenSettings,
        }
    }
}
