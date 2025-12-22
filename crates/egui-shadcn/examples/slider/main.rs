#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui_shadcn::{SliderProps, Theme, slider_with_props};

struct SliderExample {
    theme: Theme,
    value: Vec<f32>,
}

impl SliderExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: vec![50.0],
        }
    }
}

impl App for SliderExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Slider");
            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                ui.set_width(ui.available_width());
                let width = (ui.available_width() * 0.6).clamp(200.0, 420.0);
                slider_with_props(
                    ui,
                    &self.theme,
                    SliderProps::new("slider-demo", &mut self.value)
                        .min(0.0)
                        .max(100.0)
                        .step(1.0)
                        .width(width),
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Slider example",
        options,
        Box::new(|_cc| Ok(Box::new(SliderExample::new()))),
    )
}
