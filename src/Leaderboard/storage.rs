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

    #[cfg(test)]
    pub(crate) fn replace_data_for_tests(&mut self, data: LeaderboardData) {
        self.data = data;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leaderboard::data::{LeaderboardData, MatchHistoryEntry, MatchHistoryMode};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("head_soccer_{label}_{unique}.json"))
    }

    #[test]
    fn missing_file_loads_empty_store() {
        let path = temp_path("missing");
        let store = LeaderboardStore::load_or_default(&path).unwrap();

        assert_eq!(store.snapshot(), LeaderboardData::default());
    }

    #[test]
    fn invalid_json_falls_back_to_empty_store() {
        let path = temp_path("invalid");
        fs::write(&path, "{not json").unwrap();

        let store = LeaderboardStore::load_or_default(&path).unwrap();

        assert_eq!(store.snapshot(), LeaderboardData::default());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn persist_then_reload_round_trips_data() {
        let path = temp_path("roundtrip");
        let mut store = LeaderboardStore::load_or_default(&path).unwrap();
        store.replace_data_for_tests(LeaderboardData {
            solo_bot_wins: 7,
            solo_bot_losses: 3,
            matches: vec![MatchHistoryEntry {
                mode: MatchHistoryMode::Solo,
                left_score: 4,
                right_score: 1,
                recorded_at_unix_secs: 777,
            }],
        });

        store.persist().unwrap();

        let reloaded = LeaderboardStore::load_or_default(&path).unwrap();
        assert_eq!(reloaded.snapshot(), store.snapshot());

        let _ = fs::remove_file(path);
    }
}
