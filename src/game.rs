use macroquad::prelude::*;
use crate::models::ball::Ball;
use crate::models::player::Player;
use crate::physics;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    GoalScored { scorer: i32, timer: f32 },
}

pub struct Match {
    pub score_p1: i32,
    pub score_p2: i32,
    pub state: GameState,
}

impl Match {
    pub fn new() -> Self {
        Self {
            score_p1: 0,
            score_p2: 0,
            state: GameState::Playing,
        }
    }

    pub fn update(&mut self, ball: &mut Ball, players: &mut [Player]) {
        match self.state {
            GameState::Playing => {
                let left_goal_x = physics::GOAL_MARGIN_REFERENCE * physics::scale_x();
                let right_goal_x = screen_width() - physics::GOAL_MARGIN_REFERENCE * physics::scale_x();
                let crossbar_y = physics::CROSSBAR_Y_REFERENCE * physics::scale_y();

                let (bcx, bcy, bcr) = ball.circle_hitbox();

                // Détection : Sous la barre ET dépasse la ligne
                if bcy > crossbar_y {
                    if bcx < left_goal_x - bcr {
                        self.score_p2 += 1;
                        self.state = GameState::GoalScored { scorer: 2, timer: 2.0 };
                    } else if bcx > right_goal_x + bcr {
                        self.score_p1 += 1;
                        self.state = GameState::GoalScored { scorer: 1, timer: 2.0 };
                    }
                }
            }
            GameState::GoalScored { scorer, ref mut timer } => {

                *timer -= get_frame_time(); 

                
                if *timer <= 0.0 {
                    // reset de tous les joueurs
                    for player in players.iter_mut() {
                        let margin = 20.0 * physics::scale_x();
                        player.x = if player.side < 0 {
                            margin
                        } else {
                            screen_width() - player.collision_width() - margin
                        };
                        player.y = player.y_at_ground(physics::ground_level());
                        player.vx = 0.0;
                        player.vy = 0.0;
                    }

                    // reset du ballon au centre, prêt à être lancé vers le but du joueur qui a encaissé le but
                    let bcr = ball.hitbox.radius;
                    ball.x = screen_width() / 2.0;
                    
                    ball.y = physics::ground_level() - bcr - ball.hitbox.offset_y; 
                    ball.vy = 0.0;
                    ball.angle = 0.0;

                    
                    ball.vx = if scorer == 1 {
                        10.0 * physics::scale_x() 
                    } else {
                        -10.0 * physics::scale_x() 
                    };

                    // Reprise du jeu !
                    self.state = GameState::Playing;
                }
            }
        }
    }
}