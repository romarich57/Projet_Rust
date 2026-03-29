use crate::settings::data::SettingsData;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) struct SettingsStore {
    path: PathBuf,
    data: SettingsData,
}

impl SettingsStore {
    pub(crate) fn load_or_default(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        let data = match fs::read_to_string(&path) {
            Ok(raw) => serde_json::from_str::<SettingsData>(&raw)
                .ok()
                .filter(|data| data.validate().is_ok())
                .unwrap_or_default(),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => SettingsData::default(),
            Err(err) => {
                return Err(format!(
                    "failed to read settings file `{}`: {err}",
                    path.display()
                ))
            }
        };

        Ok(Self { path, data })
    }

    pub(crate) fn snapshot(&self) -> SettingsData {
        self.data
    }

    pub(crate) fn replace_and_persist(&mut self, data: SettingsData) -> Result<(), String> {
        data.validate()
            .map_err(|err| format!("invalid settings payload: {err:?}"))?;
        persist_to_path(&self.path, &data)?;
        self.data = data;
        Ok(())
    }
}

fn persist_to_path(path: &Path, data: &SettingsData) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "failed to create settings directory `{}`: {err}",
                parent.display()
            )
        })?;
    }

    let json = serde_json::to_string_pretty(data)
        .map_err(|err| format!("failed to serialize settings data: {err}"))?;
    fs::write(path, json)
        .map_err(|err| format!("failed to write settings file `{}`: {err}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::data::{ControlsConfig, PlayerBindings};
    use macroquad::prelude::KeyCode;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("head_soccer_settings_{label}_{unique}.json"))
    }

    #[test]
    fn missing_file_loads_defaults() {
        let path = temp_path("missing");

        let store = SettingsStore::load_or_default(&path).unwrap();

        assert_eq!(store.snapshot(), SettingsData::default());
    }

    #[test]
    fn invalid_json_falls_back_to_defaults() {
        let path = temp_path("invalid");
        fs::write(&path, "{not json").unwrap();

        let store = SettingsStore::load_or_default(&path).unwrap();

        assert_eq!(store.snapshot(), SettingsData::default());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn invalid_semantic_payload_falls_back_to_defaults() {
        let path = temp_path("semantic");
        let invalid = serde_json::json!({
            "controls": {
                "solo": {
                    "move_left": "Q",
                    "move_right": "D",
                    "jump": "Z",
                    "shoot": "S"
                },
                "one_vs_one_p1": {
                    "move_left": "Q",
                    "move_right": "D",
                    "jump": "Z",
                    "shoot": "S"
                },
                "one_vs_one_p2": {
                    "move_left": "Q",
                    "move_right": "Right",
                    "jump": "Up",
                    "shoot": "RightControl"
                }
            }
        });
        fs::write(&path, serde_json::to_string_pretty(&invalid).unwrap()).unwrap();

        let store = SettingsStore::load_or_default(&path).unwrap();

        assert_eq!(store.snapshot(), SettingsData::default());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn persist_then_reload_round_trips() {
        let path = temp_path("roundtrip");
        let mut store = SettingsStore::load_or_default(&path).unwrap();
        let custom = SettingsData {
            controls: ControlsConfig {
                solo: PlayerBindings {
                    move_left: KeyCode::A,
                    move_right: KeyCode::E,
                    jump: KeyCode::W,
                    shoot: KeyCode::F,
                },
                ..ControlsConfig::default()
            },
        };

        store.replace_and_persist(custom).unwrap();

        let reloaded = SettingsStore::load_or_default(&path).unwrap();
        assert_eq!(reloaded.snapshot(), custom);

        let _ = fs::remove_file(path);
    }
}
