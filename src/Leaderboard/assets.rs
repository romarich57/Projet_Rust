use crate::arcade_ui::{load_linear_texture, load_processed_texture};
use macroquad::prelude::*;

pub(crate) struct LeaderboardAssets {
    pub(crate) background: Texture2D,
    pub(crate) victory_banner: Texture2D,
    pub(crate) defeat_banner: Texture2D,
    pub(crate) history_banner: Texture2D,
    pub(crate) return_button: Texture2D,
    pub(crate) mode_header: Texture2D,
    pub(crate) score_header: Texture2D,
    pub(crate) solo_badge: Texture2D,
    pub(crate) one_vs_one_badge: Texture2D,
}

impl LeaderboardAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            background: load_linear_texture("src/assets/Leaderboard/fond.png").await?,
            victory_banner: load_processed_texture("src/assets/Leaderboard/victoire.png", false)
                .await?,
            defeat_banner: load_processed_texture("src/assets/Leaderboard/défaite.png", false)
                .await?,
            history_banner: load_processed_texture("src/assets/Leaderboard/historique.png", false)
                .await?,
            return_button: load_processed_texture("src/assets/Leaderboard/retour.png", false)
                .await?,
            mode_header: load_processed_texture("src/assets/Leaderboard/mode.png", false).await?,
            score_header: load_processed_texture("src/assets/Leaderboard/score.png", false)
                .await?,
            solo_badge: load_processed_texture("src/assets/Leaderboard/solo.png", false).await?,
            one_vs_one_badge: load_processed_texture("src/assets/Leaderboard/1V1.png", false)
                .await?,
        })
    }
}
