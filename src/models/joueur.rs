use macroquad::prelude::*; 
// Macroquad provides rendering, input and timing for this 2D game.

#[derive(Clone, Copy)]
pub struct HitboxRect {
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub height: f32,
}

// Auto-derive common traits:
// - Copy/Clone: allow cheap duplication (no move semantics issues)
// - Debug: enables {:?} printing
// - PartialEq/Eq: enables == comparisons
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlType {
    Player1,
    Player2,
    IA,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub jump_count: u8,
    pub foot_angle: f32,
    pub is_shooting: bool,
    pub head_texture: Texture2D,
    pub foot_texture: Texture2D,
    pub foot_width: f32,
    pub foot_height: f32,
    pub head_width: f32,
    pub head_height: f32,
    pub head_offset_x: f32,
    pub head_offset_y: f32,
    pub foot_hitbox: HitboxRect,
    pub head_hitbox: HitboxRect,
    pub control_type: ControlType,
    pub side: i32, // -1 for left player, +1 for right player to determine facing direction
}

impl Player {
    
    pub fn new(x: f32, y: f32, tex_t: Texture2D, tex_p: Texture2D, control_type: ControlType, side: i32) -> Self {
        Self {
            x, y,
            vx: 0.0, vy: 0.0,
            jump_count: 0,
            foot_angle: 0.0,
            is_shooting: false,
            head_texture: tex_t,
            foot_texture: tex_p,
            foot_width: 200.0,
            foot_height: 85.0,
            head_width: 200.0,
            head_height: 170.0,
            head_offset_x: 5.0,
            head_offset_y: -95.0,
            foot_hitbox: HitboxRect {
                offset_x: 45.0,
                offset_y: 20.0,
                width: 120.0,
                height: 55.0,
            },
            head_hitbox: HitboxRect {
                offset_x: 55.0,
                offset_y: 10.0,
                width: 90.0,
                height: 110.0,
            },
            control_type: control_type,
            side: side,
        }
    }

    pub fn apply_relative_screen_size(&mut self, screen_width: f32, screen_height: f32) {
        let target_width = screen_width * 0.20;
        let target_height = screen_height * 0.20;

        let base_foot_width: f32 = 200.0;
        let base_foot_height: f32 = 85.0;
        let base_head_width: f32 = 200.0;
        let base_head_height: f32 = 170.0;
        let base_head_offset_x: f32 = 5.0;
        let base_head_offset_y: f32 = -95.0;

        let base_collision_width = base_foot_width.max(base_head_offset_x + base_head_width);
        let base_total_height = base_foot_height - base_head_offset_y;

        let scale = (target_width / base_collision_width)
            .min(target_height / base_total_height);

        self.foot_width = base_foot_width * scale;
        self.foot_height = base_foot_height * scale;

        self.head_width = base_head_width * scale;
        self.head_height = base_head_height * scale;

        self.head_offset_x = base_head_offset_x * scale;
        self.head_offset_y = base_head_offset_y * scale;

        self.foot_hitbox.offset_x = 45.0 * scale;
        self.foot_hitbox.offset_y = 20.0 * scale;
        self.foot_hitbox.width = 120.0 * scale;
        self.foot_hitbox.height = 55.0 * scale;

        self.head_hitbox.offset_x = 55.0 * scale;
        self.head_hitbox.offset_y = 10.0 * scale;
        self.head_hitbox.width = 90.0 * scale;
        self.head_hitbox.height = 110.0 * scale;
    }

    pub fn foot_hitbox_rect(&self) -> (f32, f32, f32, f32) {
        let mut ox = self.foot_hitbox.offset_x;
        // Flip the offset if looking left (side == 1)
        if self.side == 1 {
            ox = self.foot_width - ox - self.foot_hitbox.width;
        }
        (
            self.x + ox,
            self.y + self.foot_hitbox.offset_y,
            self.foot_hitbox.width,
            self.foot_hitbox.height,
        )
    }

    pub fn active_foot_hitbox_rect(&self) -> (f32, f32, f32, f32) {
        let mut ox = self.foot_hitbox.offset_x;
        if self.side == 1 {
            ox = self.foot_width - ox - self.foot_hitbox.width;
        }
        
        let mut hx = self.x + ox;
        let mut hy = self.y + self.foot_hitbox.offset_y;
        let mut hw = self.foot_hitbox.width;
        let mut hh = self.foot_hitbox.height;

        // During a kick, move and stretch the foot hitbox to match the animation.
        let shot_progress = (-self.foot_angle).clamp(0.0, 1.0);
        if shot_progress > 0.05 {
            // Forward movement depends on facing direction
            if self.side == -1 {
                hx += hw * 0.38 * shot_progress; // kick right
            } else {
                hx -= hw * 0.38 * shot_progress; // kick left
            }
            hy -= hh * 0.20 * shot_progress;
            hw *= 1.0 + 0.28 * shot_progress;
            hh *= 1.0 + 0.12 * shot_progress;
        }

        (hx, hy, hw, hh)
    }

    pub fn head_hitbox_rect(&self) -> (f32, f32, f32, f32) {
        let mut ox = self.head_hitbox.offset_x;
        if self.side == 1 {
            ox = self.head_width - ox - self.head_hitbox.width;
        }
        (
            self.x + self.head_offset_x + ox,
            self.y + self.head_offset_y + self.head_hitbox.offset_y,
            self.head_hitbox.width,
            self.head_hitbox.height,
        )
    }

    /// Rectangle covering the player body between head and foot hitboxes.
    /// Prevents the ball from tunneling through the gap.
    pub fn body_hitbox_rect(&self) -> (f32, f32, f32, f32) {
        let (head_x, head_y, head_w, head_h) = self.head_hitbox_rect();
        let (foot_x, foot_y, foot_w, _foot_h) = self.foot_hitbox_rect();

        // X: union of head and foot horizontal spans
        let body_left = head_x.min(foot_x);
        let body_right = (head_x + head_w).max(foot_x + foot_w);

        // Y: from bottom of head hitbox to top of foot hitbox
        let body_top = head_y + head_h;
        let body_bottom = foot_y;

        let body_w = body_right - body_left;
        let body_h = (body_bottom - body_top).max(0.0);

        (body_left, body_top, body_w, body_h)
    }

    pub fn set_foot_hitbox(&mut self, offset_x: f32, offset_y: f32, width: f32, height: f32) {
        self.foot_hitbox.offset_x = offset_x;
        self.foot_hitbox.offset_y = offset_y;
        self.foot_hitbox.width = width;
        self.foot_hitbox.height = height;
    }

    pub fn set_head_hitbox(&mut self, offset_x: f32, offset_y: f32, width: f32, height: f32) {
        self.head_hitbox.offset_x = offset_x;
        self.head_hitbox.offset_y = offset_y;
        self.head_hitbox.width = width;
        self.head_hitbox.height = height;
    }

    pub fn collision_width(&self) -> f32 {
        self.foot_width.max(self.head_offset_x + self.head_width)
    }

    pub fn y_at_ground(&self, ground_y: f32) -> f32 {
        ground_y - self.foot_height + self.foot_height * 0.3
    }
}