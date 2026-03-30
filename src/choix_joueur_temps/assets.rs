use crate::arcade_ui::{load_linear_texture, load_processed_texture};
use macroquad::prelude::*;

pub(crate) struct MatchSetupAssets {
    pub(crate) background: Texture2D,
    pub(crate) title: Texture2D,
    pub(crate) play_button: Texture2D,
    pub(crate) back_button: Texture2D,
    pub(crate) left_fiorio: Texture2D,
    pub(crate) left_dejonckere: Texture2D,
    pub(crate) right_fiorio: Texture2D,
    pub(crate) right_dejonckere: Texture2D,
}

impl MatchSetupAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            background: load_linear_texture("src/assets/Choix_joueur_temps/fond.png").await?,
            title: load_processed_texture("src/assets/Choix_joueur_temps/choix_joueurs.png", false)
                .await?,
            play_button: load_processed_texture("src/assets/Choix_joueur_temps/jouer.png", true)
                .await?,
            back_button: load_processed_texture("src/assets/Choix_joueur_temps/retour.png", false)
                .await?,
            left_fiorio: load_processed_texture(
                "src/assets/joueur/joueur_gauche/fiorio1.png",
                false,
            )
            .await?,
            left_dejonckere: load_processed_texture(
                "src/assets/joueur/joueur_gauche/dejonckere1.png",
                false,
            )
            .await?,
            right_fiorio: load_processed_texture(
                "src/assets/joueur/joueur_droite/fiorio.png",
                false,
            )
            .await?,
            right_dejonckere: load_processed_texture(
                "src/assets/joueur/joueur_droite/dejonckere.png",
                false,
            )
            .await?,
        })
    }
}
