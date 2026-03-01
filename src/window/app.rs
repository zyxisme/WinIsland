use std::sync::Arc;
use std::time::Duration;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::{Window, WindowId, WindowLevel};

use crate::core::config::{AppConfig, PADDING, TOP_OFFSET, WINDOW_TITLE};
use crate::core::persistence::load_config;
use crate::core::render::draw_island;
use crate::utils::blur::calculate_blur_sigmas;
use crate::utils::color::get_island_border_weights;
use crate::utils::mouse::{get_global_cursor_pos, is_point_in_rect};
use crate::utils::physics::Spring;
use crate::window::tray::{TrayAction, TrayManager};

pub struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    tray: Option<TrayManager>,

    config: AppConfig,
    expanded: bool,
    tools_view: bool,
    visible: bool,
    border_weights: [f32; 4],
    target_border_weights: [f32; 4],

    spring_w: Spring,
    spring_h: Spring,
    spring_r: Spring,
    spring_view: Spring,

    os_w: u32,
    os_h: u32,

    win_x: i32,
    win_y: i32,

    frame_count: u64,
}

impl Default for App {
    fn default() -> Self {
        let config = load_config();
        Self {
            window: None,
            surface: None,
            tray: None,
            config: config.clone(),
            expanded: false,
            tools_view: false,
            visible: true,
            border_weights: [0.0; 4],
            target_border_weights: [0.0; 4],
            spring_w: Spring::new(config.base_width * config.global_scale),
            spring_h: Spring::new(config.base_height * config.global_scale),
            spring_r: Spring::new((config.base_height * config.global_scale) / 2.0),
            spring_view: Spring::new(0.0),
            os_w: 0,
            os_h: 0,
            win_x: 0,
            win_y: 0,
            frame_count: 0,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        if self.window.is_none() {
            self.os_w = (self.config.expanded_width * self.config.global_scale + PADDING) as u32;
            self.os_h = (self.config.expanded_height * self.config.global_scale + PADDING) as u32;

            let attrs = Window::default_attributes()
                .with_title(WINDOW_TITLE)
                .with_inner_size(PhysicalSize::new(self.os_w, self.os_h))
                .with_transparent(true)
                .with_decorations(false)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_skip_taskbar(true);

            let window = Arc::new(event_loop.create_window(attrs).unwrap());
            self.window = Some(window.clone());

            if let Some(monitor) = window.current_monitor() {
                let mon_size = monitor.size();
                let mon_pos = monitor.position();
                let center_x = mon_pos.x + (mon_size.width as i32) / 2;
                let top_y = mon_pos.y + TOP_OFFSET;

                self.win_x = center_x - (self.os_w as i32) / 2;
                self.win_y = top_y - (PADDING / 2.0) as i32;
                window.set_outer_position(PhysicalPosition::new(self.win_x, self.win_y));
            }

            let context = Context::new(window.clone()).unwrap();
            let mut surface = Surface::new(&context, window.clone()).unwrap();

            surface
                .resize(
                    std::num::NonZeroU32::new(self.os_w).unwrap(),
                    std::num::NonZeroU32::new(self.os_h).unwrap(),
                )
                .unwrap();
            self.surface = Some(surface);

            self.tray = Some(TrayManager::new());

            window.request_redraw();
        }
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(win) = &self.window {
            if win.id() == id {
                match event {
                    WindowEvent::CloseRequested => (),
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        let (px, py) = get_global_cursor_pos();
                        let rel_x = px - self.win_x;
                        let rel_y = py - self.win_y;

                        let island_y = PADDING as f64 / 2.0;
                        let offset_x = (self.os_w as f64 - self.spring_w.value as f64) / 2.0;

                        if is_point_in_rect(
                            rel_x as f64,
                            rel_y as f64,
                            offset_x,
                            island_y,
                            self.spring_w.value as f64,
                            self.spring_h.value as f64,
                        ) {
                            if self.expanded {
                                let center_y = island_y + self.spring_h.value as f64 / 2.0;
                                if !self.tools_view {
                                    
                                    let btn_x = offset_x + self.spring_w.value as f64 - 20.0;
                                    let dist_sq = (rel_x as f64 - btn_x).powi(2) + (rel_y as f64 - center_y).powi(2);
                                    if dist_sq <= 25.0f64.powi(2) {
                                        self.tools_view = true;
                                        self.spring_view.velocity *= 0.2;
                                        return;
                                    }
                                } else {
                                    
                                    let btn_x = offset_x + 20.0;
                                    let dist_sq = (rel_x as f64 - btn_x).powi(2) + (rel_y as f64 - center_y).powi(2);
                                    if dist_sq <= 25.0f64.powi(2) {
                                        self.tools_view = false;
                                        self.spring_view.velocity *= 0.2;
                                        return;
                                    }
                                    
                                    let grid_w = self.spring_w.value - 80.0;
                                    let grid_h = self.spring_h.value - 40.0;
                                    let x_step = (grid_w / 5.0) as f64;
                                    let y_step = (grid_h / 3.0) as f64;
                                    let start_x = offset_x + 40.0 + x_step / 2.0;
                                    let start_y = island_y + 20.0 + y_step / 2.0;
                                    
                                    let settings_cx = start_x + (0.0 * x_step);
                                    let settings_cy = start_y + (0.0 * y_step);
                                    
                                    let dist_sq = (rel_x as f64 - settings_cx as f64).powi(2) + (rel_y as f64 - settings_cy as f64).powi(2);
                                    if dist_sq <= 28.0f64.powi(2) {
                                        let _ = std::process::Command::new(std::env::current_exe().unwrap())
                                            .arg("--settings")
                                            .spawn();
                                        return;
                                    }
                                }

                                if (rel_y as f64) < island_y + 40.0 {
                                    self.expanded = false;
                                    self.tools_view = false;
                                    self.spring_w.velocity *= 0.2;
                                    self.spring_h.velocity *= 0.2;
                                    self.spring_r.velocity *= 0.2;
                                }
                            } else {
                                self.expanded = true;
                                self.spring_w.velocity *= 0.2;
                                self.spring_h.velocity *= 0.2;
                                self.spring_r.velocity *= 0.2;
                            }
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        if let Some(surface) = self.surface.as_mut() {
                            let sigmas = if self.config.motion_blur {
                                calculate_blur_sigmas(
                                    self.spring_w.velocity, 
                                    self.spring_h.velocity, 
                                    self.spring_view.velocity,
                                    self.spring_w.value
                                )
                            } else {
                                (0.0, 0.0)
                            };
                            
                            let total_w = (self.config.expanded_width - self.config.base_width).abs().max(1.0) * self.config.global_scale;
                            let dist_w = (self.spring_w.value - self.config.base_width * self.config.global_scale).abs();
                            let progress = (dist_w / total_w).clamp(0.0, 1.0);

                            draw_island(
                                surface,
                                self.spring_w.value,
                                self.spring_h.value,
                                self.spring_r.value,
                                self.os_w,
                                self.os_h,
                                self.border_weights,
                                sigmas,
                                progress,
                                self.spring_view.value,
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            if let Some(tray) = &self.tray {
                if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
                    match TrayAction::from_id(event.id, tray) {
                        Some(TrayAction::ToggleVisibility) => {
                            self.visible = !self.visible;
                            window.set_visible(self.visible);
                            tray.update_item_text(self.visible);
                        }
                        Some(TrayAction::OpenSettings) => {
                            let _ = std::process::Command::new(std::env::current_exe().unwrap())
                                .arg("--settings")
                                .spawn();
                        }
                        Some(TrayAction::Exit) => {
                            event_loop.exit();
                        }
                        None => (),
                    }
                }
            }

            let current_config = load_config();
            if current_config != self.config {
                self.config = current_config;
                let new_os_w = (self.config.expanded_width * self.config.global_scale + PADDING) as u32;
                let new_os_h = (self.config.expanded_height * self.config.global_scale + PADDING) as u32;
                if new_os_w != self.os_w || new_os_h != self.os_h {
                    self.os_w = new_os_w;
                    self.os_h = new_os_h;
                    let _ = window.request_inner_size(PhysicalSize::new(self.os_w, self.os_h));
                    if let Some(surface) = self.surface.as_mut() {
                        let _ = surface.resize(
                            std::num::NonZeroU32::new(self.os_w).unwrap(),
                            std::num::NonZeroU32::new(self.os_h).unwrap(),
                        );
                    }
                }
            }

            if !self.visible {
                std::thread::sleep(Duration::from_millis(16));
                return;
            }

            let (px, py) = get_global_cursor_pos();
            let rel_x = px - self.win_x;
            let rel_y = py - self.win_y;
            let island_y = PADDING as f64 / 2.0;
            let offset_x = (self.os_w as f64 - self.spring_w.value as f64) / 2.0;
            let is_hovering = is_point_in_rect(
                rel_x as f64,
                rel_y as f64,
                offset_x,
                island_y,
                self.spring_w.value as f64,
                self.spring_h.value as f64,
            );
            let _ = window.set_cursor_hittest(is_hovering);

            if self.config.adaptive_border {
                if self.frame_count % 30 == 0 {
                    let island_cx = self.win_x + (self.os_w as i32 / 2);
                    let island_cy =
                        self.win_y + (PADDING as i32 / 2) + (self.spring_h.value as i32 / 2);
                    let raw_weights = get_island_border_weights(
                        island_cx,
                        island_cy,
                        self.spring_w.value,
                        self.spring_h.value,
                    );
                    self.target_border_weights = raw_weights.map(|w| if w > 0.85 { w } else { 0.0 });
                }
            } else {
                self.target_border_weights = [0.0; 4];
            }
            self.frame_count += 1;

            let mut border_changed = false;
            for i in 0..4 {
                let diff = self.target_border_weights[i] - self.border_weights[i];
                if diff.abs() > 0.005 {
                    self.border_weights[i] += diff * 0.1;
                    border_changed = true;
                } else {
                    self.border_weights[i] = self.target_border_weights[i];
                }
            }

            if border_changed {
                window.request_redraw();
            }

            let target_w = (if self.expanded {
                self.config.expanded_width
            } else {
                self.config.base_width
            }) * self.config.global_scale;
            let target_h = (if self.expanded {
                self.config.expanded_height
            } else {
                self.config.base_height
            }) * self.config.global_scale;
            let target_r = if self.expanded {
                32.0 * self.config.global_scale
            } else {
                (self.config.base_height * self.config.global_scale) / 2.0
            };

            let target_view = if self.tools_view { 1.0 } else { 0.0 };

            let total_w = (self.config.expanded_width - self.config.base_width)
                .abs()
                .max(1.0) * self.config.global_scale;
            let dist_w = (target_w - self.spring_w.value).abs();
            let ratio = (dist_w / total_w).clamp(0.0, 1.0);

            let (stiffness, damping) = if self.expanded {
                let s = 0.11 * (1.0 - ratio * 0.6).max(0.4);
                (s, 0.70)
            } else {
                (0.11, 0.65)
            };

            self.spring_w.update(target_w, stiffness, damping);
            self.spring_h.update(target_h, stiffness, damping);
            self.spring_r.update(target_r, stiffness, damping);
            self.spring_view.update(target_view, 0.12, 0.58);

            if self.spring_w.velocity.abs() > 0.01
                || self.spring_h.velocity.abs() > 0.01
                || self.spring_r.velocity.abs() > 0.01
                || self.spring_view.velocity.abs() > 0.001
            {
                window.request_redraw();
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
