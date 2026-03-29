use crate::arcade_ui::{load_linear_texture, load_processed_texture};
use macroquad::prelude::*;

pub(crate) struct SettingsAssets {
    pub(crate) background: Texture2D,
    pub(crate) title_banner: Texture2D,
    pub(crate) solo_banner: Texture2D,
    pub(crate) one_vs_one_banner: Texture2D,
    pub(crate) reset_button: Texture2D,
    pub(crate) save_button: Texture2D,
    pub(crate) return_button: Texture2D,
    pub(crate) settings_icon: Texture2D,
}

impl SettingsAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            background: load_linear_texture("src/assets/paramètre/fond.png").await?,
            title_banner: load_processed_texture("src/assets/paramètre/Parametre.png", false)
                .await?,
            solo_banner: load_processed_texture("src/assets/paramètre/mode_solo.png", false)
                .await?,
            one_vs_one_banner: load_processed_texture(
                "src/assets/paramètre/mode_2_joueurs.png",
                false,
            )
            .await?,
            reset_button: load_processed_texture(
                "src/assets/paramètre/reinitialiser_lestouches.png",
                false,
            )
            .await?,
            save_button: load_processed_texture("src/assets/paramètre/sauvegarder.png", false)
                .await?,
            return_button: load_processed_texture("src/assets/paramètre/retour.png", false).await?,
            settings_icon: load_processed_texture("src/assets/paramètre/parametre_1.png", true)
                .await?,
        })
    }
}
