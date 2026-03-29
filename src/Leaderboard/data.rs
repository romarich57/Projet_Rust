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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_win_increments_wins_and_prepends_history() {
        let mut data = LeaderboardData::default();

        data.record_match(MatchMode::Solo, 3, 1, 111);

        assert_eq!(data.solo_bot_wins, 1);
        assert_eq!(data.solo_bot_losses, 0);
        assert_eq!(data.matches.len(), 1);
        assert_eq!(data.matches[0].mode, MatchHistoryMode::Solo);
        assert_eq!(data.matches[0].left_score, 3);
        assert_eq!(data.matches[0].right_score, 1);
        assert_eq!(data.matches[0].recorded_at_unix_secs, 111);
    }

    #[test]
    fn solo_loss_increments_losses() {
        let mut data = LeaderboardData::default();

        data.record_match(MatchMode::Solo, 1, 4, 222);

        assert_eq!(data.solo_bot_wins, 0);
        assert_eq!(data.solo_bot_losses, 1);
    }

    #[test]
    fn solo_draw_only_updates_history() {
        let mut data = LeaderboardData::default();

        data.record_match(MatchMode::Solo, 2, 2, 333);

        assert_eq!(data.solo_bot_wins, 0);
        assert_eq!(data.solo_bot_losses, 0);
        assert_eq!(data.matches.len(), 1);
    }

    #[test]
    fn one_vs_one_does_not_touch_solo_stats() {
        let mut data = LeaderboardData {
            solo_bot_wins: 4,
            solo_bot_losses: 2,
            matches: Vec::new(),
        };

        data.record_match(MatchMode::OneVsOne, 5, 4, 444);

        assert_eq!(data.solo_bot_wins, 4);
        assert_eq!(data.solo_bot_losses, 2);
        assert_eq!(data.matches.len(), 1);
        assert_eq!(data.matches[0].mode, MatchHistoryMode::OneVsOne);
    }

    #[test]
    fn history_is_capped_to_twenty_most_recent_matches() {
        let mut data = LeaderboardData::default();

        for recorded_at in 0..25 {
            data.record_match(MatchMode::OneVsOne, recorded_at as u8, 0, recorded_at);
        }

        assert_eq!(data.matches.len(), MAX_HISTORY_ENTRIES);
        assert_eq!(data.matches.first().unwrap().recorded_at_unix_secs, 24);
        assert_eq!(data.matches.last().unwrap().recorded_at_unix_secs, 5);
    }
}
