use skia_safe::{Canvas, Color, Paint, PathBuilder, paint};

pub fn draw_arrow_right(canvas: &Canvas, cx: f32, cy: f32, alpha: u8) {
    let mut paint = Paint::default();
    paint.set_color(Color::from_argb(alpha, 200, 200, 200));
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::Stroke);
    paint.set_stroke_width(2.0);
    paint.set_stroke_cap(paint::Cap::Round);
    paint.set_stroke_join(paint::Join::Round);

    let mut builder = PathBuilder::new();
    builder.move_to((cx - 4.0, cy - 6.0));
    builder.line_to((cx + 2.0, cy));
    builder.line_to((cx - 4.0, cy + 6.0));
    let path = builder.detach();
    canvas.draw_path(&path, &paint);
}

pub fn draw_arrow_left(canvas: &Canvas, cx: f32, cy: f32, alpha: u8) {
    let mut paint = Paint::default();
    paint.set_color(Color::from_argb(alpha, 200, 200, 200));
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::Stroke);
    paint.set_stroke_width(2.0);
    paint.set_stroke_cap(paint::Cap::Round);
    paint.set_stroke_join(paint::Join::Round);

    let mut builder = PathBuilder::new();
    builder.move_to((cx + 2.0, cy - 6.0));
    builder.line_to((cx - 4.0, cy));
    builder.line_to((cx + 2.0, cy + 6.0));
    let path = builder.detach();
    canvas.draw_path(&path, &paint);
}
