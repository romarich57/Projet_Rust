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

