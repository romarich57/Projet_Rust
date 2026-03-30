use crate::arcade_ui::{load_linear_texture, load_processed_texture};
use macroquad::prelude::*;

pub(crate) struct MenuAssets {
    pub(crate) background: Texture2D,
    pub(crate) logo: Texture2D,
    pub(crate) messi: Texture2D,
    pub(crate) ronaldo: Texture2D,
    pub(crate) play_button: Texture2D,
    pub(crate) scoreboard_button: Texture2D,
    pub(crate) quit_button: Texture2D,
    pub(crate) settings_icon: Texture2D,
}

impl MenuAssets {
    pub(crate) async fn load() -> Result<Self, String> {
        Ok(Self {
            background: load_linear_texture("src/assets/menu/fond.png").await?,
            logo: load_processed_texture("src/assets/menu/headsoccer.png", true).await?,
            messi: load_processed_texture("src/assets/menu/messi.png", false).await?,
            ronaldo: load_processed_texture("src/assets/menu/Ronaldo.png", false).await?,
            play_button: load_processed_texture("src/assets/menu/jouer.png", true).await?,
            scoreboard_button: load_processed_texture("src/assets/menu/scoreboard.png", false)
                .await?,
            quit_button: load_processed_texture("src/assets/menu/quitter.png", true).await?,
            settings_icon: load_processed_texture("src/assets/menu/parametre_1.png", false).await?,
        })
    }
}
