use skia_safe::{Canvas, Color, Paint, Path};

pub fn draw_play_button(canvas: &Canvas, cx: f32, cy: f32, alpha: u8, scale: f32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    paint.set_style(skia_safe::paint::Style::Fill);

    let path_data = "M187.2 100.9C174.8 94.1 159.8 94.4 147.6 101.6C135.4 108.8 128 121.9 128 136L128 504C128 518.1 135.5 531.2 147.6 538.4C159.7 545.6 174.8 545.9 187.2 539.1L523.2 355.1C536 348.1 544 334.6 544 320C544 305.4 536 291.9 523.2 284.9L187.2 100.9z";

    if let Some(path) = Path::from_svg(path_data) {
        canvas.save();
        canvas.translate((cx, cy));
        let s = 0.09 * scale;
        canvas.scale((s, s));
        canvas.translate((-320.0, -320.0));
        canvas.draw_path(&path, &paint);
        canvas.restore();
    }
}

pub fn draw_pause_button(canvas: &Canvas, cx: f32, cy: f32, alpha: u8, scale: f32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    paint.set_style(skia_safe::paint::Style::Fill);

    let path_data = "M176 96C149.5 96 128 117.5 128 144L128 496C128 522.5 149.5 544 176 544L240 544C266.5 544 288 522.5 288 496L288 144C288 117.5 266.5 96 240 96L176 96zM400 96C373.5 96 352 117.5 352 144L352 496C352 522.5 373.5 544 400 544L464 544C490.5 544 512 522.5 512 496L512 144C512 117.5 490.5 96 464 96L400 96z";

    if let Some(path) = Path::from_svg(path_data) {
        canvas.save();
        canvas.translate((cx, cy));
        let s = 0.09 * scale;
        canvas.scale((s, s));
        canvas.translate((-320.0, -320.0));
        canvas.draw_path(&path, &paint);
        canvas.restore();
    }
}

pub fn draw_next_button(canvas: &Canvas, cx: f32, cy: f32, alpha: u8, scale: f32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    paint.set_style(skia_safe::paint::Style::Fill);

    let path_data = "M12.053 3.212C11.729 2.898 11.288 2.7 10.8 2.7C9.806 2.7 9 3.507 9 4.501L9 9.001L9 13.501C9 14.495 9.806 15.3 10.8 15.3C11.288 15.3 11.73 15.105 12.053 14.788C14.434 12.474 18 9.001 18 9.001S14.434 5.527 12.053 3.212zM3.053 3.212C2.729 2.898 2.288 2.7 1.8 2.7C0.806 2.7 0 3.507 0 4.501L0 13.501C0 14.495 0.806 15.3 1.8 15.3C2.288 15.3 2.73 15.105 3.053 14.788C5.434 12.474 9 9.001 9 9.001S5.434 5.527 3.053 3.212z";

    if let Some(path) = Path::from_svg(path_data) {
        canvas.save();
        canvas.translate((cx, cy));
        let s = 2.4 * scale;
        canvas.scale((s, s));
        canvas.translate((-9.0, -9.0));
        canvas.draw_path(&path, &paint);
        canvas.restore();
    }
}

pub fn draw_prev_button(canvas: &Canvas, cx: f32, cy: f32, alpha: u8, scale: f32) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(Color::from_argb(alpha, 255, 255, 255));
    paint.set_style(skia_safe::paint::Style::Fill);

    let path_data = "M12.053 3.212C11.729 2.898 11.288 2.7 10.8 2.7C9.806 2.7 9 3.507 9 4.501L9 9.001L9 13.501C9 14.495 9.806 15.3 10.8 15.3C11.288 15.3 11.73 15.105 12.053 14.788C14.434 12.474 18 9.001 18 9.001S14.434 5.527 12.053 3.212zM3.053 3.212C2.729 2.898 2.288 2.7 1.8 2.7C0.806 2.7 0 3.507 0 4.501L0 13.501C0 14.495 0.806 15.3 1.8 15.3C2.288 15.3 2.73 15.105 3.053 14.788C5.434 12.474 9 9.001 9 9.001S5.434 5.527 3.053 3.212z";

    if let Some(path) = Path::from_svg(path_data) {
        canvas.save();
        canvas.translate((cx, cy));
        let s = 2.4 * scale;
        canvas.scale((-s, s));
        canvas.translate((-9.0, -9.0));
        canvas.draw_path(&path, &paint);
        canvas.restore();
    }
}
