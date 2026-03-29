use macroquad::file::load_file;
use macroquad::prelude::*;
use resvg::{tiny_skia, usvg};
use std::collections::VecDeque;

const ALPHA_TRIM_THRESHOLD: u8 = 8;

pub(crate) async fn load_linear_texture(path: &str) -> Result<Texture2D, String> {
    let texture = load_texture(path)
        .await
        .map_err(|err| format!("failed to load texture `{path}`: {err}"))?;
    texture.set_filter(FilterMode::Linear);
    Ok(texture)
}

pub(crate) async fn load_processed_texture(
    path: &str,
    strip_white_matte: bool,
) -> Result<Texture2D, String> {
    let bytes = load_file(path)
        .await
        .map_err(|err| format!("failed to read texture bytes `{path}`: {err}"))?;
    let mut image = Image::from_file_with_format(&bytes, None)
        .map_err(|err| format!("failed to decode image `{path}`: {err}"))?;

    if strip_white_matte {
        remove_edge_white_matte(&mut image);
    }

    let trimmed = crop_useful_content(&image, ALPHA_TRIM_THRESHOLD)
        .ok_or_else(|| format!("texture `{path}` became empty after trimming"))?;
    let texture = Texture2D::from_image(&trimmed);
    texture.set_filter(FilterMode::Linear);
    Ok(texture)
}

pub(crate) async fn load_svg_texture(path: &str, target_px: UVec2) -> Result<Texture2D, String> {
    let bytes = load_file(path)
        .await
        .map_err(|err| format!("failed to read svg bytes `{path}`: {err}"))?;
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(&bytes, &options)
        .map_err(|err| format!("failed to parse svg `{path}`: {err}"))?;

    let width = target_px.x.max(1);
    let height = target_px.y.max(1);
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| format!("failed to allocate svg pixmap `{path}`"))?;
    let source_size = tree.size().to_int_size();
    let transform = tiny_skia::Transform::from_scale(
        width as f32 / source_size.width() as f32,
        height as f32 / source_size.height() as f32,
    );

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let mut rgba = pixmap.data().to_vec();
    unpremultiply_rgba(&mut rgba);
    let texture = Texture2D::from_rgba8(width as u16, height as u16, &rgba);
    texture.set_filter(FilterMode::Linear);
    Ok(texture)
}

pub(crate) fn crop_useful_content(image: &Image, alpha_threshold: u8) -> Option<Image> {
    let width = image.width();
    let height = image.height();
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_image_data()[y * width + x];
            if pixel[3] < alpha_threshold {
                continue;
            }

            found = true;
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    if !found {
        return None;
    }

    Some(image.sub_image(Rect::new(
        min_x as f32,
        min_y as f32,
        (max_x - min_x + 1) as f32,
        (max_y - min_y + 1) as f32,
    )))
}

pub(crate) fn remove_edge_white_matte(image: &mut Image) {
    let width = image.width();
    let height = image.height();
    let mut visited = vec![false; width * height];
    let mut queue = VecDeque::new();

    {
        let pixels = image.get_image_data();

        for x in 0..width {
            enqueue_if_edge_matte(x, 0, width, pixels, &mut visited, &mut queue);
            enqueue_if_edge_matte(x, height - 1, width, pixels, &mut visited, &mut queue);
        }

        for y in 0..height {
            enqueue_if_edge_matte(0, y, width, pixels, &mut visited, &mut queue);
            enqueue_if_edge_matte(width - 1, y, width, pixels, &mut visited, &mut queue);
        }
    }

    let pixels = image.get_image_data_mut();
    while let Some((x, y)) = queue.pop_front() {
        let idx = y * width + x;
        pixels[idx][3] = 0;

        for (nx, ny) in neighbors(x, y, width, height) {
            let nidx = ny * width + nx;
            if visited[nidx] || !is_near_white_matte(pixels[nidx]) {
                continue;
            }

            visited[nidx] = true;
            queue.push_back((nx, ny));
        }
    }
}

pub(crate) fn fit_contain(slot: Rect, texture_width: f32, texture_height: f32) -> Rect {
    let scale = (slot.w / texture_width).min(slot.h / texture_height);
    let width = texture_width * scale;
    let height = texture_height * scale;

    Rect::new(
        slot.x + (slot.w - width) * 0.5,
        slot.y + (slot.h - height) * 0.5,
        width,
        height,
    )
}

pub(crate) fn compute_cover_source_rect(
    texture_width: f32,
    texture_height: f32,
    target_width: f32,
    target_height: f32,
) -> Rect {
    let texture_aspect = texture_width / texture_height;
    let target_aspect = target_width / target_height;

    if texture_aspect > target_aspect {
        let source_width = texture_height * target_aspect;
        let source_x = (texture_width - source_width) * 0.5;
        Rect::new(source_x, 0.0, source_width, texture_height)
    } else {
        let source_height = texture_width / target_aspect;
        let source_y = (texture_height - source_height) * 0.5;
        Rect::new(0.0, source_y, texture_width, source_height)
    }
}

pub(crate) fn draw_cover_texture(texture: &Texture2D, dest: Rect) {
    let source = compute_cover_source_rect(texture.width(), texture.height(), dest.w, dest.h);

    draw_texture_ex(
        texture,
        dest.x,
        dest.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(dest.w, dest.h)),
            source: Some(source),
            ..Default::default()
        },
    );
}

pub(crate) fn scale_rect_from_center(rect: Rect, scale: f32) -> Rect {
    let width = rect.w * scale;
    let height = rect.h * scale;

    Rect::new(
        rect.x + (rect.w - width) * 0.5,
        rect.y + (rect.h - height) * 0.5,
        width,
        height,
    )
}

pub(crate) fn draw_slot_texture(texture: &Texture2D, slot: Rect, scale: f32) {
    let base_rect = fit_contain(slot, texture.width(), texture.height());
    let draw_rect = scale_rect_from_center(base_rect, scale);

    draw_texture_ex(
        texture,
        draw_rect.x,
        draw_rect.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(draw_rect.w, draw_rect.h)),
            ..Default::default()
        },
    );
}

fn enqueue_if_edge_matte(
    x: usize,
    y: usize,
    width: usize,
    pixels: &[[u8; 4]],
    visited: &mut [bool],
    queue: &mut VecDeque<(usize, usize)>,
) {
    let idx = y * width + x;
    if visited[idx] || !is_near_white_matte(pixels[idx]) {
        return;
    }

    visited[idx] = true;
    queue.push_back((x, y));
}

fn is_near_white_matte(pixel: [u8; 4]) -> bool {
    pixel[3] > 0 && pixel[0] > 250 && pixel[1] > 250 && pixel[2] > 250
}

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let left = x.checked_sub(1).map(|nx| (nx, y));
    let up = y.checked_sub(1).map(|ny| (x, ny));
    let right = (x + 1 < width).then_some((x + 1, y));
    let down = (y + 1 < height).then_some((x, y + 1));

    [left, up, right, down].into_iter().flatten()
}

fn unpremultiply_rgba(bytes: &mut [u8]) {
    for pixel in bytes.chunks_exact_mut(4) {
        let alpha = pixel[3] as u32;
        if alpha == 0 || alpha == 255 {
            continue;
        }

        pixel[0] = ((pixel[0] as u32 * 255) / alpha).min(255) as u8;
        pixel[1] = ((pixel[1] as u32 * 255) / alpha).min(255) as u8;
        pixel[2] = ((pixel[2] as u32 * 255) / alpha).min(255) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_edge_white_matte_preserves_internal_white() {
        let mut image = Image::gen_image_color(7, 7, color_u8!(252, 252, 252, 255));

        for y in 2..=4 {
            for x in 2..=4 {
                image.set_pixel(x, y, BLACK);
            }
        }
        image.set_pixel(3, 3, color_u8!(252, 252, 252, 255));

        remove_edge_white_matte(&mut image);

        let pixels = image.get_image_data();
        assert_eq!(pixels[0][3], 0);
        assert_eq!(pixels[3 * 7 + 3][3], 255);
        assert_eq!(pixels[2 * 7 + 2][3], 255);
    }

    #[test]
    fn crop_useful_content_ignores_low_alpha_pixels() {
        let mut image = Image::gen_image_color(10, 10, color_u8!(0, 0, 0, 0));
        image.set_pixel(0, 0, color_u8!(255, 255, 255, 7));

        for y in 2..=5 {
            for x in 4..=6 {
                image.set_pixel(x, y, color_u8!(255, 0, 0, 255));
            }
        }

        let cropped = crop_useful_content(&image, 8).expect("image should keep opaque content");

        assert_eq!(cropped.width(), 3);
        assert_eq!(cropped.height(), 4);
    }

    #[test]
    fn fit_contain_preserves_aspect_ratio() {
        let slot = Rect::new(310.0, 250.0, 380.0, 96.0);
        let fitted = fit_contain(slot, 983.0, 269.0);

        assert!((fitted.h - 96.0).abs() < 0.001);
        assert!(fitted.w <= slot.w);
        assert!((fitted.x - (slot.x + (slot.w - fitted.w) * 0.5)).abs() < 0.001);
    }

    #[test]
    fn compute_cover_source_rect_crops_height_when_texture_is_taller() {
        let source = compute_cover_source_rect(1536.0, 1024.0, 1000.0, 600.0);

        assert_eq!(source.x, 0.0);
        assert!(source.h < 1024.0);
        assert!((source.w - 1536.0).abs() < 0.001);
    }
}
