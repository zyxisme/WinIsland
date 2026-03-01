use skia_safe::{Canvas, Color, Paint};
use crate::icons::arrows::draw_arrow_left;
use crate::icons::settings::draw_settings_icon;

pub fn draw_tools_page(canvas: &Canvas, ox: f32, oy: f32, w: f32, h: f32, alpha: u8) {
    draw_arrow_left(canvas, ox + 20.0, oy + h / 2.0, alpha);
    draw_watch_grid_tools(canvas, ox, oy, w, h, alpha);
}

fn draw_watch_grid_tools(canvas: &Canvas, ox: f32, oy: f32, w: f32, h: f32, alpha: u8) {
    let grid_cols = 5;
    let grid_rows = 3;
    let bubble_r = 18.0;
    
    let grid_w = w - 80.0;
    let grid_h = h - 40.0;
    
    let start_x = ox + 40.0 + (grid_w / (grid_cols as f32)) / 2.0;
    let start_y = oy + 20.0 + (grid_h / (grid_rows as f32)) / 2.0;
    
    let x_step = grid_w / (grid_cols as f32);
    let y_step = grid_h / (grid_rows as f32);

    for r in 0..grid_rows {
        for c in 0..grid_cols {
            let cx = start_x + (c as f32 * x_step);
            let cy = start_y + (r as f32 * y_step);
            
            let is_settings = r == 0 && c == 0;
            let final_alpha = if is_settings { alpha } else { (alpha as f32 * 0.2) as u8 };
            
            draw_tool_bubble(canvas, cx, cy, bubble_r, final_alpha, |canvas, x, y, a| {
                if is_settings {
                    draw_settings_icon(canvas, x, y, a);
                }
            });
        }
    }
}

fn draw_tool_bubble<F>(canvas: &Canvas, cx: f32, cy: f32, r: f32, alpha: u8, draw_content: F)
where F: FnOnce(&Canvas, f32, f32, u8) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb((alpha as f32 * 0.15) as u8, 255, 255, 255));
    canvas.draw_circle((cx, cy), r, &paint);
    paint.set_style(skia_safe::paint::Style::Stroke);
    paint.set_stroke_width(1.0);
    paint.set_color(Color::from_argb((alpha as f32 * 0.2) as u8, 255, 255, 255));
    canvas.draw_circle((cx, cy), r, &paint);
    draw_content(canvas, cx, cy, alpha);
}
