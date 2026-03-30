use super::controls::ArrowDirection;
use crate::arcade_ui::scale_rect_from_center;
use macroquad::prelude::*;

pub(super) fn draw_neon_panel(rect: Rect) {
    draw_rectangle(
        rect.x - 6.0,
        rect.y - 6.0,
        rect.w + 12.0,
        rect.h + 12.0,
        Color::new(0.11, 0.65, 0.97, 0.09),
    );
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color_u8!(12, 28, 53, 238));
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        3.0,
        color_u8!(59, 126, 201, 255),
    );
    draw_line(
        rect.x + 14.0,
        rect.y + 16.0,
        rect.x + rect.w - 14.0,
        rect.y + 16.0,
        2.0,
        color_u8!(216, 190, 91, 255),
    );
    draw_line(
        rect.x + 14.0,
        rect.y + rect.h - 16.0,
        rect.x + rect.w - 14.0,
        rect.y + rect.h - 16.0,
        2.0,
        color_u8!(216, 190, 91, 255),
    );
}

pub(super) fn draw_inner_display(rect: Rect) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color_u8!(14, 44, 88, 230));
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        color_u8!(59, 126, 201, 255),
    );
}

pub(super) fn draw_arrow_button(
    rect: Rect,
    direction: ArrowDirection,
    hovered: bool,
    pressed: bool,
) {
    if hovered || pressed {
        let glow = scale_rect_from_center(rect, 1.10);
        draw_rectangle(
            glow.x,
            glow.y,
            glow.w,
            glow.h,
            Color::new(0.26, 0.75, 1.0, 0.18),
        );
    }

    let scale = match (hovered, pressed) {
        (_, true) => 0.96,
        (true, false) => 1.03,
        _ => 1.0,
    };
    let draw_rect = scale_rect_from_center(rect, scale);

    draw_rectangle(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        color_u8!(246, 246, 246, 255),
    );
    draw_rectangle_lines(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        2.0,
        color_u8!(212, 188, 95, 255),
    );
    draw_chevron(draw_rect, direction, color_u8!(242, 221, 129, 255));
}

pub(super) fn draw_chevron(rect: Rect, direction: ArrowDirection, color: Color) {
    let thickness = rect.w.min(rect.h) * 0.08;
    let offset_x = rect.w * 0.18;
    let mid_x = rect.x + rect.w * 0.5;
    let top_y = rect.y + rect.h * 0.28;
    let center_y = rect.y + rect.h * 0.5;
    let bottom_y = rect.y + rect.h * 0.72;

    match direction {
        ArrowDirection::Left => {
            draw_line(
                mid_x + offset_x * 0.3,
                top_y,
                mid_x - offset_x,
                center_y,
                thickness,
                color,
            );
            draw_line(
                mid_x - offset_x,
                center_y,
                mid_x + offset_x * 0.3,
                bottom_y,
                thickness,
                color,
            );
        }
        ArrowDirection::Right => {
            draw_line(
                mid_x - offset_x * 0.3,
                top_y,
                mid_x + offset_x,
                center_y,
                thickness,
                color,
            );
            draw_line(
                mid_x + offset_x,
                center_y,
                mid_x - offset_x * 0.3,
                bottom_y,
                thickness,
                color,
            );
        }
    }
}

pub(super) fn draw_difficulty_button(
    rect: Rect,
    label: &str,
    selected: bool,
    hovered: bool,
    pressed: bool,
) {
    let base_color = if selected {
        color_u8!(26, 71, 128, 245)
    } else {
        color_u8!(10, 32, 60, 222)
    };
    let border_color = if selected {
        color_u8!(243, 224, 125, 255)
    } else {
        color_u8!(87, 162, 220, 255)
    };

    if hovered || selected {
        let glow = scale_rect_from_center(rect, if selected { 1.12 } else { 1.08 });
        draw_rectangle(
            glow.x,
            glow.y,
            glow.w,
            glow.h,
            Color::new(0.22, 0.78, 1.0, if selected { 0.20 } else { 0.12 }),
        );
    }

    let scale = match (hovered, pressed) {
        (_, true) => 0.98,
        (true, false) => 1.03,
        _ => 1.0,
    };
    let draw_rect = scale_rect_from_center(rect, scale);

    draw_rectangle(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        base_color,
    );
    draw_rectangle_lines(
        draw_rect.x,
        draw_rect.y,
        draw_rect.w,
        draw_rect.h,
        2.0,
        border_color,
    );
    draw_text_centered(
        label,
        vec2(
            draw_rect.x + draw_rect.w * 0.5,
            draw_rect.y + draw_rect.h * 0.62,
        ),
        22.0,
        if selected {
            color_u8!(255, 246, 186, 255)
        } else {
            WHITE
        },
    );
}

pub(super) fn draw_text_centered(label: &str, anchor: Vec2, font_size: f32, color: Color) {
    let metrics = measure_text(label, None, font_size as u16, 1.0);
    let x = anchor.x - metrics.width * 0.5;
    let y = anchor.y;

    draw_text(
        label,
        x + 2.0,
        y + 2.0,
        font_size,
        Color::new(0.0, 0.0, 0.0, 0.45),
    );
    draw_text(label, x, y, font_size, color);
}
