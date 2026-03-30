use crate::app::SceneCommand;
use crate::gameplay::MatchMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ModeSelectionButton {
    Solo,
    OneVsOne,
    Back,
}

impl ModeSelectionButton {
    pub(super) const ALL: [Self; 3] = [Self::Solo, Self::OneVsOne, Self::Back];

    pub(super) fn command(self) -> SceneCommand {
        match self {
            Self::Solo => SceneCommand::OpenMatchSetup(MatchMode::Solo),
            Self::OneVsOne => SceneCommand::OpenMatchSetup(MatchMode::OneVsOne),
            Self::Back => SceneCommand::BackToMenu,
        }
    }
}
