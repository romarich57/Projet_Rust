use crate::gameplay::MatchMode;
use crate::leaderboard::data::LeaderboardData;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct LeaderboardStore {
    path: PathBuf,
    data: LeaderboardData,
}

impl LeaderboardStore {
    pub(crate) fn load_or_default(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();
        let data = match fs::read_to_string(&path) {
            Ok(raw) => serde_json::from_str(&raw).unwrap_or_default(),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => LeaderboardData::default(),
            Err(err) => {
                return Err(format!(
                    "failed to read leaderboard file `{}`: {err}",
                    path.display()
                ))
            }
        };

        Ok(Self { path, data })
    }

    pub(crate) fn snapshot(&self) -> LeaderboardData {
        self.data.clone()
    }

    pub(crate) fn persist(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                format!(
                    "failed to create leaderboard directory `{}`: {err}",
                    parent.display()
                )
            })?;
        }

        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|err| format!("failed to serialize leaderboard data: {err}"))?;
        fs::write(&self.path, json).map_err(|err| {
            format!(
                "failed to write leaderboard file `{}`: {err}",
                self.path.display()
            )
        })
    }

    pub(crate) fn record_match(
        &mut self,
        mode: MatchMode,
        left_score: u8,
        right_score: u8,
    ) -> Result<(), String> {
        let recorded_at_unix_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.data
            .record_match(mode, left_score, right_score, recorded_at_unix_secs);
        self.persist()
    }
}

