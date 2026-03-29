use crate::match_arena::{ArenaGeometry, GoalSide};
use crate::models::ball::Ball;
use crate::models::player::Player;
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    GoalScored { scorer: i32, timer: f32 },
    Finished,
}

pub struct Match {
    pub score_p1: i32,
    pub score_p2: i32,
    pub state: GameState,
    remaining_time_seconds: f32,
}

impl Match {
    pub fn new(length_seconds: f32) -> Self {
        Self {
            score_p1: 0,
            score_p2: 0,
            state: GameState::Playing,
            remaining_time_seconds: length_seconds.max(0.0),
        }
    }

    pub fn remaining_time_seconds(&self) -> f32 {
        self.remaining_time_seconds
    }

    pub fn score(&self) -> (i32, i32) {
        (self.score_p1, self.score_p2)
    }

    fn advance_match_clock(&mut self, delta_seconds: f32) {
        if self.state != GameState::Playing {
            return;
        }

        self.remaining_time_seconds = (self.remaining_time_seconds - delta_seconds).max(0.0);

        if self.remaining_time_seconds <= 0.0 {
            self.state = GameState::Finished;
        }
    }

    pub fn update(
        &mut self,
        ball: &mut Ball,
        players: &mut [Player],
        arena: &ArenaGeometry,
        delta_seconds: f32,
    ) {
        match self.state {
            GameState::Playing => {
                if let Some(goal_side) = arena.goal_scored(ball) {
                    let scorer = match goal_side {
                        GoalSide::Left => {
                            self.score_p2 += 1;
                            2
                        }
                        GoalSide::Right => {
                            self.score_p1 += 1;
                            1
                        }
                    };

                    self.state = GameState::GoalScored { scorer, timer: 2.0 };
                }

                if self.state == GameState::Playing {
                    self.advance_match_clock(delta_seconds);
                }
            }
            GameState::GoalScored {
                scorer,
                ref mut timer,
            } => {
                *timer -= delta_seconds;

                if *timer <= 0.0 {
                    for player in players.iter_mut() {
                        player.x = arena.player_spawn_x(player.side, player.collision_width());
                        player.y = player.y_at_ground(arena.ground_y);
                        player.vx = 0.0;
                        player.vy = 0.0;
                        player.jump_count = 0;
                        player.foot_angle = 0.0;
                        player.is_shooting = false;
                    }

                    let spawn = arena.ball_spawn_position(ball.hitbox.radius, ball.hitbox.offset_y);
                    ball.x = spawn.x;
                    ball.y = spawn.y;
                    ball.vy = 0.0;
                    ball.angle = 0.0;
                    ball.vx = if scorer == 1 {
                        8.0 * arena.uniform_scale
                    } else {
                        -8.0 * arena.uniform_scale
                    };

                    self.state = GameState::Playing;
                }
            }
            GameState::Finished => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::match_arena::ArenaGeometry;

    #[test]
    fn new_match_initializes_remaining_time() {
        let soccer_match = Match::new(120.0);

        assert!((soccer_match.remaining_time_seconds() - 120.0).abs() < f32::EPSILON);
        assert!(soccer_match.state == GameState::Playing);
    }

    #[test]
    fn timer_decreases_while_playing() {
        let mut soccer_match = Match::new(10.0);

        soccer_match.advance_match_clock(1.25);

        assert!((soccer_match.remaining_time_seconds() - 8.75).abs() < 0.001);
    }

    #[test]
    fn timer_does_not_decrease_during_goal_sequence() {
        let mut soccer_match = Match::new(10.0);
        soccer_match.state = GameState::GoalScored {
            scorer: 1,
            timer: 2.0,
        };

        soccer_match.advance_match_clock(1.0);

        assert!((soccer_match.remaining_time_seconds() - 10.0).abs() < 0.001);
    }

    #[test]
    fn match_switches_to_finished_at_zero() {
        let mut soccer_match = Match::new(0.5);

        soccer_match.advance_match_clock(0.75);

        assert!(soccer_match.state == GameState::Finished);
        assert!((soccer_match.remaining_time_seconds() - 0.0).abs() < 0.001);
    }

    #[test]
    fn score_does_not_change_after_finished() {
        let mut soccer_match = Match::new(0.0);
        soccer_match.state = GameState::Finished;

        soccer_match.advance_match_clock(1.0);

        assert_eq!(soccer_match.score_p1, 0);
        assert_eq!(soccer_match.score_p2, 0);
    }

    #[test]
    fn arena_goal_detection_distinguishes_left_and_right() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);

        let left_center = vec2(
            arena.left_goal.mouth_line_x - 24.0,
            (arena.left_goal.opening_top_y + arena.left_goal.opening_bottom_y) * 0.5,
        );
        let right_center = vec2(
            arena.right_goal.mouth_line_x + 24.0,
            (arena.right_goal.opening_top_y + arena.right_goal.opening_bottom_y) * 0.5,
        );

        assert_eq!(
            arena.goal_scored_at(left_center, 12.0),
            Some(GoalSide::Left)
        );
        assert_eq!(
            arena.goal_scored_at(right_center, 12.0),
            Some(GoalSide::Right)
        );
    }
}
