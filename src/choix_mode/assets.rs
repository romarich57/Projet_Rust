use crate::arcade_ui::{load_linear_texture, load_processed_texture};
use macroquad::prelude::*;

pub(crate) struct ModeSelectionAssets {
    pub(crate) background: Texture2D,
    pub(crate) title: Texture2D,
    pub(crate) solo_button: Texture2D,
    pub(crate) one_vs_one_button: Texture2D,
    pub(crate) back_button: Texture2D,
    pub(crate) materazzi: Texture2D,
    pub(crate) zidane: Texture2D,
}

impl ModeSelectionAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            background: load_linear_texture("src/assets/choix_mode/fond.png").await?,
            title: load_processed_texture("src/assets/choix_mode/choisisez_mode.png", false)
                .await?,
            solo_button: load_processed_texture("src/assets/choix_mode/mode_solo.png", false)
                .await?,
            one_vs_one_button: load_processed_texture("src/assets/choix_mode/mode1v1.png", false)
                .await?,
            back_button: load_processed_texture("src/assets/choix_mode/retour.png", false).await?,
            materazzi: load_processed_texture("src/assets/choix_mode/materazzi.png", false).await?,
            zidane: load_processed_texture("src/assets/choix_mode/zidane.png", false).await?,
        })
    }
}
