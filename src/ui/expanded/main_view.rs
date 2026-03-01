use skia_safe::Canvas;
use crate::icons::arrows::draw_arrow_right;

pub fn draw_main_page(canvas: &Canvas, ox: f32, oy: f32, w: f32, h: f32, alpha: u8) {
    draw_arrow_right(canvas, ox + w - 20.0, oy + h / 2.0, alpha);
}
