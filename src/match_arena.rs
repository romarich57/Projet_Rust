use crate::models::ball::Ball;
use macroquad::prelude::*;

const REFERENCE_WIDTH: f32 = 1000.0;
const REFERENCE_HEIGHT: f32 = 600.0;

const GOAL_POST_WIDTH_RATIO: f32 = 6.0 / 80.0;
const GOAL_BACK_POST_X_RATIO: f32 = 4.5 / 80.0;
const GOAL_FIELD_POST_X_RATIO: f32 = 69.5 / 80.0;
const GOAL_CROSSBAR_TOP_RATIO: f32 = 4.5 / 120.0;
const GOAL_OPENING_TOP_RATIO: f32 = 10.0 / 120.0;
const GOAL_OPENING_BOTTOM_RATIO: f32 = 111.0 / 120.0;
const GOAL_FIELD_POST_TIP_HEIGHT_RATIO: f32 = 0.15;

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
    pub(crate) goal_draw_width: f32,
    pub(crate) goal_draw_height: f32,
    pub(crate) goal_net_padding: f32,
}

impl ArenaTuning {
    pub(crate) fn reference() -> Self {
        Self {
            hud_height: 92.0,
            ground_y: 497.0,
            player_left_wall_x: 82.0,
            player_right_wall_x: 918.0,
            left_spawn_center_x: 280.0,
            right_spawn_center_x: 720.0,
            ball_spawn_center_x: 500.0,
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
    pub(crate) mouth_line_x: f32,
    pub(crate) opening_top_y: f32,
    pub(crate) opening_bottom_y: f32,
    pub(crate) field_post_tip_rect: Rect,
    pub(crate) back_post_rect: Rect,
    pub(crate) crossbar_rect: Rect,
    pub(crate) goal_floor_rect: Rect,
    pub(crate) goal_cavity_rect: Rect,
    pub(crate) goal_capture_rect: Rect,
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
        let ground_y = tuning.ground_y * scale_y;

        let goal_draw_width = tuning.goal_draw_width * uniform_scale;
        let goal_draw_height = tuning.goal_draw_height * uniform_scale;
        let goal_draw_y = ground_y - goal_draw_height * 0.935;

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
            ground_y,
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
        let post_top_y = draw_rect.y + draw_rect.h * GOAL_CROSSBAR_TOP_RATIO;

        let back_post_x = match side {
            GoalSide::Left => draw_rect.x + draw_rect.w * GOAL_BACK_POST_X_RATIO,
            GoalSide::Right => draw_rect.x + draw_rect.w * GOAL_FIELD_POST_X_RATIO,
        };
        let field_post_x = match side {
            GoalSide::Left => draw_rect.x + draw_rect.w * GOAL_FIELD_POST_X_RATIO,
            GoalSide::Right => draw_rect.x + draw_rect.w * GOAL_BACK_POST_X_RATIO,
        };

        let field_post_tip_rect = Rect::new(
            field_post_x,
            post_top_y,
            post_width,
            (opening_bottom_y - opening_top_y) * GOAL_FIELD_POST_TIP_HEIGHT_RATIO,
        );
        let back_post_rect = Rect::new(
            back_post_x,
            post_top_y,
            post_width,
            opening_bottom_y - draw_rect.y,
        );

        let crossbar_x = back_post_rect.x.min(field_post_tip_rect.x);
        let crossbar_right = (back_post_rect.x + back_post_rect.w)
            .max(field_post_tip_rect.x + field_post_tip_rect.w);
        let crossbar_rect = Rect::new(
            crossbar_x,
            post_top_y,
            crossbar_right - crossbar_x,
            crossbar_height.max(4.0),
        );

        let mouth_line_x = field_post_tip_rect.x + field_post_tip_rect.w * 0.5;

        let goal_cavity_rect = match side {
            GoalSide::Left => Rect::new(
                draw_rect.x + net_padding,
                opening_top_y + net_padding,
                (mouth_line_x - draw_rect.x - net_padding).max(1.0),
                (opening_bottom_y - opening_top_y - net_padding * 2.0).max(1.0),
            ),
            GoalSide::Right => Rect::new(
                mouth_line_x,
                opening_top_y + net_padding,
                (draw_rect.right() - mouth_line_x - net_padding).max(1.0),
                (opening_bottom_y - opening_top_y - net_padding * 2.0).max(1.0),
            ),
        };
        let capture_inset = (net_padding * 0.6).max(4.0);
        let goal_capture_rect = Rect::new(
            goal_cavity_rect.x + capture_inset,
            goal_cavity_rect.y + capture_inset,
            (goal_cavity_rect.w - capture_inset * 2.0).max(1.0),
            (goal_cavity_rect.h - capture_inset * 2.0).max(1.0),
        );
        let goal_floor_rect = Rect::new(
            goal_cavity_rect.x,
            opening_bottom_y,
            goal_cavity_rect.w,
            (draw_rect.bottom() - opening_bottom_y).max(2.0),
        );

        GoalGeometry {
            side,
            draw_rect,
            mouth_line_x,
            opening_top_y,
            opening_bottom_y,
            field_post_tip_rect,
            back_post_rect,
            crossbar_rect,
            goal_floor_rect,
            goal_cavity_rect,
            goal_capture_rect,
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
        if Self::is_ball_captured_in_goal(center, radius, self.left_goal) {
            return Some(GoalSide::Left);
        }

        if Self::is_ball_captured_in_goal(center, radius, self.right_goal) {
            return Some(GoalSide::Right);
        }

        None
    }

    pub(crate) fn ball_in_goal_net(&self, ball: &Ball) -> Option<GoalSide> {
        let (center_x, center_y, radius) = ball.circle_hitbox();
        let center = vec2(center_x, center_y);

        if rect_circle_overlap(self.left_goal.goal_cavity_rect, center, radius) {
            return Some(GoalSide::Left);
        }

        if rect_circle_overlap(self.right_goal.goal_cavity_rect, center, radius) {
            return Some(GoalSide::Right);
        }

        None
    }

    fn is_ball_captured_in_goal(center: Vec2, radius: f32, goal: GoalGeometry) -> bool {
        if !Self::has_crossed_goal_mouth(center, radius, goal) {
            return false;
        }

        rect_contains_point(goal.goal_capture_rect, center)
    }

    fn has_crossed_goal_mouth(center: Vec2, radius: f32, goal: GoalGeometry) -> bool {
        let within_opening = center.y > goal.opening_top_y + radius * 0.2
            && center.y < goal.opening_bottom_y - radius * 0.15;

        if !within_opening {
            return false;
        }

        match goal.side {
            GoalSide::Left => center.x < goal.mouth_line_x - radius * 0.2,
            GoalSide::Right => center.x > goal.mouth_line_x + radius * 0.2,
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

fn rect_contains_point(rect: Rect, point: Vec2) -> bool {
    point.x >= rect.x && point.x <= rect.right() && point.y >= rect.y && point.y <= rect.bottom()
}

