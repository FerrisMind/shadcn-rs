#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::{CentralPanel, Color32};
use egui_shadcn::{ProgressProps, ProgressSize, ProgressVariant, Theme, progress};

struct ProgressExample {
    theme: Theme,
    progress_value: f32,
}

impl ProgressExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            progress_value: 60.0,
        }
    }
}

impl App for ProgressExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Progress Component");
            ui.add_space(16.0);

            ui.label("Determinate progress:");
            ui.add(egui::Slider::new(&mut self.progress_value, 0.0..=100.0).text("Value"));
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(self.progress_value)),
            );

            ui.add_space(24.0);
            ui.label("Sizes:");
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(75.0)).size(ProgressSize::Size1),
            );
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(75.0)).size(ProgressSize::Size2),
            );
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(75.0)).size(ProgressSize::Size3),
            );

            ui.add_space(24.0);
            ui.label("Variants:");
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(60.0)).variant(ProgressVariant::Classic),
            );
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(60.0)).variant(ProgressVariant::Surface),
            );
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(60.0)).variant(ProgressVariant::Soft),
            );

            ui.add_space(24.0);
            ui.label("Custom colors:");
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(80.0)).color(Color32::from_rgb(34, 197, 94)),
            );
            ui.add_space(8.0);
            progress(
                ui,
                &self.theme,
                ProgressProps::new(Some(45.0)).color(Color32::from_rgb(239, 68, 68)),
            );

            ui.add_space(24.0);
            ui.label("Indeterminate (loading):");
            ui.add_space(8.0);
            progress(ui, &self.theme, ProgressProps::new(None));
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Progress example",
        options,
        Box::new(|_cc| Ok(Box::new(ProgressExample::new()))),
    )
}
