use skia_safe::{Color, Paint, Rect, RRect, surfaces, gradient_shader, Point, image_filters};
use softbuffer::Surface;
use std::sync::Arc;
use winit::window::Window;
use crate::core::config::PADDING;
use crate::ui::expanded::main_view::draw_main_page;
use crate::ui::expanded::tools_view::draw_tools_page;

pub fn draw_island(
    surface: &mut Surface<Arc<Window>, Arc<Window>>,
    current_w: f32,
    current_h: f32,
    current_r: f32,
    os_w: u32,
    os_h: u32,
    weights: [f32; 4],
    sigmas: (f32, f32),
    expansion_progress: f32,
    view_offset: f32,
) {
    let mut buffer = surface.buffer_mut().unwrap();
    let mut sk_surface = surfaces::raster_n32_premul(skia_safe::ISize::new(os_w as i32, os_h as i32)).unwrap();
    let canvas = sk_surface.canvas();
    canvas.clear(Color::TRANSPARENT);

    let offset_x = (os_w as f32 - current_w) / 2.0;
    let offset_y = PADDING / 2.0;
    
    let rect = Rect::from_xywh(
        offset_x, 
        offset_y, 
        current_w, 
        current_h
    );
    let rrect = RRect::new_rect_xy(rect, current_r, current_r);

    
    let has_blur = sigmas.0 > 0.1 || sigmas.1 > 0.1;
    let blur_filter = if has_blur {
        image_filters::blur(sigmas, None, None, None)
    } else {
        None
    };

    
    canvas.save();
    canvas.clip_rrect(rrect, skia_safe::ClipOp::Intersect, true);

    
    let mut bg_paint = Paint::default();
    bg_paint.set_color(Color::BLACK);
    bg_paint.set_anti_alias(true);
    if has_blur {
        if let Some(ref filter) = blur_filter {
            let mut layer_paint = Paint::default();
            layer_paint.set_image_filter(filter.clone());
            canvas.save_layer(&skia_safe::canvas::SaveLayerRec::default().paint(&layer_paint));
            canvas.draw_rrect(rrect, &bg_paint);
            canvas.restore();
        } else {
            canvas.draw_rrect(rrect, &bg_paint);
        }
    } else {
        canvas.draw_rrect(rrect, &bg_paint);
    }

    
    if expansion_progress > 0.01 {
        let alpha_factor = (expansion_progress.powf(2.5)).clamp(0.0, 1.0);
        let alpha = (alpha_factor * 255.0) as u8;
        
        if has_blur {
            if let Some(ref filter) = blur_filter {
                let mut layer_paint = Paint::default();
                layer_paint.set_image_filter(filter.clone());
                canvas.save_layer(&skia_safe::canvas::SaveLayerRec::default().paint(&layer_paint));
            }
        }

        
        canvas.save();
        canvas.translate((-view_offset * current_w, 0.0));
        draw_main_page(canvas, offset_x, offset_y, current_w, current_h, alpha);
        canvas.restore();

        
        canvas.save();
        canvas.translate(((1.0 - view_offset) * current_w, 0.0));
        draw_tools_page(canvas, offset_x, offset_y, current_w, current_h, alpha);
        canvas.restore();

        if has_blur {
            canvas.restore();
        }
    }

    
    let total_weight: f32 = weights.iter().sum();
    if total_weight > 0.01 {
        let center = Point::new(os_w as f32 / 2.0, offset_y + current_h / 2.0);
        let colors = [
            if weights[0] > 0.85 { Color::from_argb((weights[0] * 100.0) as u8, 255, 255, 255) } else { Color::TRANSPARENT },
            if weights[1] > 0.85 { Color::from_argb((weights[1] * 100.0) as u8, 255, 255, 255) } else { Color::TRANSPARENT },
            if weights[2] > 0.85 { Color::from_argb((weights[2] * 100.0) as u8, 255, 255, 255) } else { Color::TRANSPARENT },
            if weights[3] > 0.85 { Color::from_argb((weights[3] * 100.0) as u8, 255, 255, 255) } else { Color::TRANSPARENT },
            if weights[0] > 0.85 { Color::from_argb((weights[0] * 100.0) as u8, 255, 255, 255) } else { Color::TRANSPARENT },
        ];
        let stops = [0.0, 0.25, 0.5, 0.75, 1.0];
        if let Some(shader) = gradient_shader::sweep(center, &colors[..], Some(&stops[..]), skia_safe::TileMode::Clamp, None, None, None) {
            let mut stroke_paint = Paint::default();
            stroke_paint.set_shader(shader);
            stroke_paint.set_style(skia_safe::paint::Style::Stroke);
            stroke_paint.set_stroke_width(1.3);
            stroke_paint.set_anti_alias(true);
            if has_blur {
                if let Some(ref filter) = blur_filter {
                    stroke_paint.set_image_filter(filter.clone());
                }
            }
            canvas.draw_rrect(rrect, &stroke_paint);
        }
    }

    canvas.restore(); 

    let info = skia_safe::ImageInfo::new(skia_safe::ISize::new(os_w as i32, os_h as i32), skia_safe::ColorType::BGRA8888, skia_safe::AlphaType::Premul, None);
    let dst_row_bytes = (os_w * 4) as usize;
    let u8_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut *buffer);
    let _ = sk_surface.read_pixels(&info, u8_buffer, dst_row_bytes, (0, 0));
    buffer.present().unwrap();
}
