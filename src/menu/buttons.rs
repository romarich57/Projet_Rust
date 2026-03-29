use crate::app::SceneCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MenuButton {
    Play,
    Scoreboard,
    Quit,
}

impl MenuButton {
    pub(super) const ALL: [Self; 3] = [Self::Play, Self::Scoreboard, Self::Quit];

    pub(super) fn command(self) -> SceneCommand {
        match self {
            Self::Play => SceneCommand::OpenModeSelection,
            Self::Scoreboard => SceneCommand::OpenScoreboard,
            Self::Quit => SceneCommand::Quit,
        }
    }
}
