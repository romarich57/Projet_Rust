use super::assets::MatchSetupAssets;
use crate::gameplay::PlayerProfile;
use macroquad::prelude::Texture2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MatchSetupControl {
    LeftPlayerPrev,
    LeftPlayerNext,
    RightPlayerPrev,
    RightPlayerNext,
    LengthPrev,
    LengthNext,
    DifficultyEasy,
    DifficultyNormal,
    DifficultyHard,
    Back,
    Play,
}

#[derive(Clone, Copy)]
pub(super) enum ArrowDirection {
    Left,
    Right,
}

pub(super) fn left_portrait_texture<'a>(
    assets: &'a MatchSetupAssets,
    profile: PlayerProfile,
) -> &'a Texture2D {
    match profile {
        PlayerProfile::Fiorio => &assets.left_fiorio,
        PlayerProfile::Dejonckere => &assets.left_dejonckere,
    }
}

pub(super) fn right_portrait_texture<'a>(
    assets: &'a MatchSetupAssets,
    profile: PlayerProfile,
) -> &'a Texture2D {
    match profile {
        PlayerProfile::Fiorio => &assets.right_fiorio,
        PlayerProfile::Dejonckere => &assets.right_dejonckere,
    }
}
