use crate::gameplay::MatchMode;
use serde::{Deserialize, Serialize};

pub(crate) const MAX_HISTORY_ENTRIES: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum MatchHistoryMode {
    Solo,
    OneVsOne,
}

impl From<MatchMode> for MatchHistoryMode {
    fn from(value: MatchMode) -> Self {
        match value {
            MatchMode::Solo => Self::Solo,
            MatchMode::OneVsOne => Self::OneVsOne,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MatchHistoryEntry {
    pub(crate) mode: MatchHistoryMode,
    pub(crate) left_score: u8,
    pub(crate) right_score: u8,
    pub(crate) recorded_at_unix_secs: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LeaderboardData {
    pub(crate) solo_bot_wins: u32,
    pub(crate) solo_bot_losses: u32,
    pub(crate) matches: Vec<MatchHistoryEntry>,
}

impl LeaderboardData {
    pub(crate) fn record_match(
        &mut self,
        mode: MatchMode,
        left_score: u8,
        right_score: u8,
        recorded_at_unix_secs: u64,
    ) {
        if matches!(mode, MatchMode::Solo) {
            if left_score > right_score {
                self.solo_bot_wins += 1;
            } else if left_score < right_score {
                self.solo_bot_losses += 1;
            }
        }

        self.matches.insert(
            0,
            MatchHistoryEntry {
                mode: mode.into(),
                left_score,
                right_score,
                recorded_at_unix_secs,
            },
        );
        self.matches.truncate(MAX_HISTORY_ENTRIES);
    }
}

