use macroquad::prelude::*;

pub(super) fn draw_vs_text(center: Vec2, font_size: f32) {
    let label = "VS";
    let metrics = measure_text(label, None, font_size as u16, 1.0);
    let x = center.x - metrics.width * 0.5;
    let y = center.y;

    draw_text(
        label,
        x + 5.0,
        y + 6.0,
        font_size,
        Color::new(0.35, 0.12, 0.0, 0.85),
    );
    draw_text(
        label,
        x + 2.0,
        y + 2.0,
        font_size,
        Color::new(1.0, 0.74, 0.15, 0.62),
    );
    draw_text(label, x, y, font_size, Color::new(1.0, 0.96, 0.80, 1.0));
}
