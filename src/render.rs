use macroquad::prelude::*;
use crate::models::ball::Ball;
use crate::models::player::Player;
use crate::game::{GameState, Match};

fn draw_score(game_match: &Match) {
    let score_text = format!("{}  -  {}", game_match.score_p1, game_match.score_p2);
    let font_size = 75.0; 
    let text_params = measure_text(&score_text, None, font_size as u16, 1.0);
    

    let box_width = text_params.width + 100.0;
    let box_height = text_params.height + 30.0;
    let box_x = screen_width() / 2.0 - box_width / 2.0;
    let box_y = 10.0;


    draw_rectangle(
        box_x + 5.0, box_y + 5.0, 
        box_width, box_height, 
        Color::new(0.0, 0.0, 0.0, 0.4)
    );
    
 
    draw_rectangle(
        box_x, box_y, 
        box_width, box_height, 
        Color::new(0.15, 0.15, 0.15, 1.0)
    );
    

    draw_rectangle_lines(
        box_x, box_y, 
        box_width, box_height, 
        4.0,
        Color::new(0.7, 0.7, 0.7, 1.0)
    );

  
    let text_x = screen_width() / 2.0 - text_params.width / 2.0;
    let text_y = box_y + text_params.height + 8.0;

   
    draw_text(&score_text, text_x + 3.0, text_y + 3.0, font_size, BLACK);
    
    
    draw_text(&score_text, text_x, text_y, font_size, WHITE);

    //animation de but
    if let GameState::GoalScored { .. } = game_match.state {
        let goal_text = "GOAL !";
        let goal_size = 140.0; 
        let goal_params = measure_text(goal_text, None, goal_size as u16, 1.0);
        
        let text_x = screen_width() / 2.0 - goal_params.width / 2.0;
        let text_y = screen_height() / 2.0 + goal_params.height / 2.0 - 50.0;

        draw_text(goal_text, text_x + 8.0, text_y + 8.0, goal_size, BLACK);
        draw_text(goal_text, text_x, text_y, goal_size, YELLOW);
    }
}


pub fn draw_all(players: &[Player], stadium_texture: &Texture2D, ball: &Ball, debug_hitbox: bool, game_match: &Match) {
    draw_texture_ex(stadium_texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    });

    draw_texture_ex(
        &ball.texture,
        ball.x - ball.visual_radius(), 
        ball.y - ball.visual_radius(),
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(ball.visual_radius() * 2.0, ball.visual_radius() * 2.0)),
            rotation: ball.angle,
            ..Default::default()
        },
    );

    for player in players {
        let mut foot_draw_x = player.x;
        // If facing left (side == -1), drawing with negative width flips the image LEFT of 'foot_draw_x'.
        // So we must shift 'foot_draw_x' right by foot_width to keep the image within its actual bounds.
        if player.side == -1 {
            foot_draw_x += player.foot_width;
        }

        draw_texture_ex(
            &player.foot_texture,
            foot_draw_x,
            player.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(player.foot_width * player.side as f32, player.foot_height)),
                rotation: player.foot_angle,
                ..Default::default()
            },
        );

        let mut head_draw_x = player.x + player.head_offset_x;
        // Invert horizontal rendering width for orientation:
        // if player.side == -1, drawing with negative width flips the image.
        if player.side == -1 {
            head_draw_x += player.head_width;
        }

        draw_texture_ex(
            &player.head_texture,
            head_draw_x,
            player.y + player.head_offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(player.head_width * player.side as f32, player.head_height)),
                ..Default::default()
            },
        );
    }

    draw_score(game_match);

    if debug_hitbox {
        for player in players {
            draw_debug_hitbox(player, ball);
        }
        draw_text("DEBUG HITBOX (Y)", 15.0, 30.0, 26.0, YELLOW);
    }
}

fn draw_debug_hitbox(player: &Player, ball: &Ball) {
    let (foot_base_x, foot_base_y, foot_base_w, foot_base_h) = player.foot_hitbox_rect();
    let (foot_x, foot_y, foot_w, foot_h) = player.active_foot_hitbox_rect();
    let (head_x, head_y, head_w, head_h) = player.head_hitbox_rect();
    let (body_x, body_y, body_w, body_h) = player.body_hitbox_rect();

    let ball_hitbox = ball.circle_hitbox();
    let ball_cx = ball_hitbox.0;
    let ball_cy = ball_hitbox.1;
    let ball_r = ball_hitbox.2;

    draw_rectangle_lines(foot_base_x, foot_base_y, foot_base_w, foot_base_h, 1.0, PINK);
    draw_rectangle_lines(foot_x, foot_y, foot_w, foot_h, 2.0, RED);
    draw_rectangle_lines(head_x, head_y, head_w, head_h, 2.0, ORANGE);
    if body_h > 0.0 {
        draw_rectangle_lines(body_x, body_y, body_w, body_h, 2.0, BLUE);
    }
    draw_rectangle_lines(
        ball_cx - ball_r,
        ball_cy - ball_r,
        ball_r * 2.0,
        ball_r * 2.0,
        2.0,
        LIME,
    );
}