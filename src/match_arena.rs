use crate::models::ball::Ball;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

const GOAL_POST_WIDTH_RATIO: f32 = 6.0 / 80.0;
const GOAL_BACK_POST_X_RATIO: f32 = 4.5 / 80.0;
const GOAL_FRONT_POST_X_RATIO: f32 = 69.5 / 80.0;
const GOAL_CROSSBAR_TOP_RATIO: f32 = 4.5 / 120.0;
const GOAL_OPENING_TOP_RATIO: f32 = 10.0 / 120.0;
const GOAL_OPENING_BOTTOM_RATIO: f32 = 111.0 / 120.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GoalSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ArenaTuning {
    pub(crate) hud_height: f32,
    pub(crate) ground_y: f32,
    pub(crate) player_left_wall_x: f32,
    pub(crate) player_right_wall_x: f32,
    pub(crate) left_spawn_center_x: f32,
    pub(crate) right_spawn_center_x: f32,
    pub(crate) ball_spawn_center_x: f32,
    pub(crate) goal_draw_y: f32,
    pub(crate) goal_draw_width: f32,
    pub(crate) goal_draw_height: f32,
    pub(crate) goal_net_padding: f32,
}

impl ArenaTuning {
    pub(crate) fn reference() -> Self {
        Self {
            hud_height: 92.0,
            ground_y: 522.0,
            player_left_wall_x: 82.0,
            player_right_wall_x: 918.0,
            left_spawn_center_x: 280.0,
            right_spawn_center_x: 720.0,
            ball_spawn_center_x: 500.0,
            goal_draw_y: 355.0,
            goal_draw_width: 128.0,
            goal_draw_height: 192.0,
            goal_net_padding: 10.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GoalGeometry {
    pub(crate) side: GoalSide,
    pub(crate) draw_rect: Rect,
    pub(crate) goal_line_x: f32,
    pub(crate) opening_top_y: f32,
    pub(crate) opening_bottom_y: f32,
    pub(crate) front_post_rect: Rect,
    pub(crate) back_post_rect: Rect,
    pub(crate) crossbar_rect: Rect,
    pub(crate) net_zone_rect: Rect,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ArenaGeometry {
    pub(crate) screen_width: f32,
    pub(crate) screen_height: f32,
    pub(crate) uniform_scale: f32,
    pub(crate) hud_height: f32,
    pub(crate) ground_y: f32,
    pub(crate) player_left_wall_x: f32,
    pub(crate) player_right_wall_x: f32,
    pub(crate) left_spawn_center_x: f32,
    pub(crate) right_spawn_center_x: f32,
    pub(crate) ball_spawn_center_x: f32,
    pub(crate) left_goal: GoalGeometry,
    pub(crate) right_goal: GoalGeometry,
}

impl ArenaGeometry {
    pub(crate) fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        let tuning = ArenaTuning::reference();
        let scale_x = screen_width / REFERENCE_WIDTH;
        let scale_y = screen_height / REFERENCE_HEIGHT;
        let uniform_scale = scale_x.min(scale_y);

        let goal_draw_width = tuning.goal_draw_width * uniform_scale;
        let goal_draw_height = tuning.goal_draw_height * uniform_scale;
        let goal_draw_y = tuning.goal_draw_y * scale_y;

        let left_goal_rect = Rect::new(0.0, goal_draw_y, goal_draw_width, goal_draw_height);
        let right_goal_rect = Rect::new(
            screen_width - goal_draw_width,
            goal_draw_y,
            goal_draw_width,
            goal_draw_height,
        );

        Self {
            screen_width,
            screen_height,
            uniform_scale,
            hud_height: tuning.hud_height * scale_y,
            ground_y: tuning.ground_y * scale_y,
            player_left_wall_x: tuning.player_left_wall_x * scale_x,
            player_right_wall_x: tuning.player_right_wall_x * scale_x,
            left_spawn_center_x: tuning.left_spawn_center_x * scale_x,
            right_spawn_center_x: tuning.right_spawn_center_x * scale_x,
            ball_spawn_center_x: tuning.ball_spawn_center_x * scale_x,
            left_goal: Self::build_goal(
                GoalSide::Left,
                left_goal_rect,
                tuning.goal_net_padding * uniform_scale,
            ),
            right_goal: Self::build_goal(
                GoalSide::Right,
                right_goal_rect,
                tuning.goal_net_padding * uniform_scale,
            ),
        }
    }

    fn build_goal(side: GoalSide, draw_rect: Rect, net_padding: f32) -> GoalGeometry {
        let post_width = draw_rect.w * GOAL_POST_WIDTH_RATIO;
        let crossbar_height =
            draw_rect.h * GOAL_OPENING_TOP_RATIO - draw_rect.h * GOAL_CROSSBAR_TOP_RATIO;
        let opening_top_y = draw_rect.y + draw_rect.h * GOAL_OPENING_TOP_RATIO;
        let opening_bottom_y = draw_rect.y + draw_rect.h * GOAL_OPENING_BOTTOM_RATIO;

        let back_post_x = match side {
            GoalSide::Left => draw_rect.x + draw_rect.w * GOAL_BACK_POST_X_RATIO,
            GoalSide::Right => {
                draw_rect.x + draw_rect.w * (1.0 - GOAL_FRONT_POST_X_RATIO) - post_width
            }
        };
        let front_post_x = match side {
            GoalSide::Left => draw_rect.x + draw_rect.w * GOAL_FRONT_POST_X_RATIO,
            GoalSide::Right => {
                draw_rect.x + draw_rect.w * (1.0 - GOAL_BACK_POST_X_RATIO) - post_width
            }
        };

        let front_post_rect = Rect::new(
            front_post_x,
            draw_rect.y + draw_rect.h * GOAL_CROSSBAR_TOP_RATIO,
            post_width,
            opening_bottom_y - draw_rect.y,
        );
        let back_post_rect = Rect::new(
            back_post_x,
            draw_rect.y + draw_rect.h * GOAL_CROSSBAR_TOP_RATIO,
            post_width,
            opening_bottom_y - draw_rect.y,
        );

        let crossbar_x = back_post_rect.x.min(front_post_rect.x);
        let crossbar_right =
            (back_post_rect.x + back_post_rect.w).max(front_post_rect.x + front_post_rect.w);
        let crossbar_rect = Rect::new(
            crossbar_x,
            draw_rect.y + draw_rect.h * GOAL_CROSSBAR_TOP_RATIO,
            crossbar_right - crossbar_x,
            crossbar_height.max(4.0),
        );

        let goal_line_x = match side {
            GoalSide::Left => front_post_rect.x + front_post_rect.w * 0.5,
            GoalSide::Right => front_post_rect.x + front_post_rect.w * 0.5,
        };

        let net_zone_rect = match side {
            GoalSide::Left => Rect::new(
                draw_rect.x + net_padding,
                opening_top_y + net_padding,
                (goal_line_x - draw_rect.x - net_padding).max(1.0),
                (opening_bottom_y - opening_top_y - net_padding * 2.0).max(1.0),
            ),
            GoalSide::Right => Rect::new(
                goal_line_x,
                opening_top_y + net_padding,
                (draw_rect.right() - goal_line_x - net_padding).max(1.0),
                (opening_bottom_y - opening_top_y - net_padding * 2.0).max(1.0),
            ),
        };

        GoalGeometry {
            side,
            draw_rect,
            goal_line_x,
            opening_top_y,
            opening_bottom_y,
            front_post_rect,
            back_post_rect,
            crossbar_rect,
            net_zone_rect,
        }
    }

    pub(crate) fn player_spawn_x(&self, side: i32, collision_width: f32) -> f32 {
        let center_x = if side < 0 {
            self.left_spawn_center_x
        } else {
            self.right_spawn_center_x
        };

        (center_x - collision_width * 0.5).clamp(
            self.player_left_wall_x,
            self.player_right_wall_x - collision_width,
        )
    }

    pub(crate) fn ball_spawn_position(&self, radius: f32, offset_y: f32) -> Vec2 {
        vec2(self.ball_spawn_center_x, self.ground_y - radius - offset_y)
    }

    pub(crate) fn goal_scored(&self, ball: &Ball) -> Option<GoalSide> {
        let (center_x, center_y, radius) = ball.circle_hitbox();
        self.goal_scored_at(vec2(center_x, center_y), radius)
    }

    pub(crate) fn goal_scored_at(&self, center: Vec2, radius: f32) -> Option<GoalSide> {
        if Self::is_ball_past_goal_line(center, radius, self.left_goal) {
            return Some(GoalSide::Left);
        }

        if Self::is_ball_past_goal_line(center, radius, self.right_goal) {
            return Some(GoalSide::Right);
        }

        None
    }

    pub(crate) fn ball_in_goal_net(&self, ball: &Ball) -> Option<GoalSide> {
        let (center_x, center_y, radius) = ball.circle_hitbox();
        let center = vec2(center_x, center_y);

        if rect_circle_overlap(self.left_goal.net_zone_rect, center, radius) {
            return Some(GoalSide::Left);
        }

        if rect_circle_overlap(self.right_goal.net_zone_rect, center, radius) {
            return Some(GoalSide::Right);
        }

        None
    }

    fn is_ball_past_goal_line(center: Vec2, radius: f32, goal: GoalGeometry) -> bool {
        let within_opening = center.y > goal.opening_top_y + radius * 0.2
            && center.y < goal.opening_bottom_y - radius * 0.15;

        if !within_opening {
            return false;
        }

        match goal.side {
            GoalSide::Left => center.x < goal.goal_line_x - radius * 0.35,
            GoalSide::Right => center.x > goal.goal_line_x + radius * 0.35,
        }
    }
}

fn rect_circle_overlap(rect: Rect, center: Vec2, radius: f32) -> bool {
    let closest_x = center.x.clamp(rect.x, rect.right());
    let closest_y = center.y.clamp(rect.y, rect.bottom());
    let dx = center.x - closest_x;
    let dy = center.y - closest_y;

    dx * dx + dy * dy <= radius * radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_stays_inside_screen_bounds() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);

        assert!(arena.hud_height < arena.left_goal.draw_rect.y);
        assert!(arena.left_goal.draw_rect.x >= 0.0);
        assert!(arena.right_goal.draw_rect.right() <= arena.screen_width + 0.001);
        assert!(arena.ground_y < arena.screen_height);
    }

    #[test]
    fn spawns_stay_on_each_side_of_center() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);

        assert!(arena.left_spawn_center_x < arena.ball_spawn_center_x);
        assert!(arena.right_spawn_center_x > arena.ball_spawn_center_x);
    }

    #[test]
    fn goals_do_not_overlap_hud() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);

        assert!(arena.left_goal.draw_rect.y > arena.hud_height);
        assert!(arena.right_goal.draw_rect.y > arena.hud_height);
    }

    #[test]
    fn goal_is_scored_only_inside_opening() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);
        let center = vec2(
            arena.left_goal.goal_line_x - 20.0,
            (arena.left_goal.opening_top_y + arena.left_goal.opening_bottom_y) * 0.5,
        );

        assert_eq!(arena.goal_scored_at(center, 12.0), Some(GoalSide::Left));
    }

    #[test]
    fn goal_is_not_scored_above_crossbar() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);
        let center = vec2(
            arena.left_goal.goal_line_x - 20.0,
            arena.left_goal.opening_top_y - 18.0,
        );

        assert_eq!(arena.goal_scored_at(center, 12.0), None);
    }

    #[test]
    fn touching_post_only_is_not_a_goal() {
        let arena = ArenaGeometry::from_screen(1000.0, 600.0);
        let center = vec2(
            arena.left_goal.goal_line_x - 2.0,
            (arena.left_goal.opening_top_y + arena.left_goal.opening_bottom_y) * 0.5,
        );

        assert_eq!(arena.goal_scored_at(center, 12.0), None);
    }
}
