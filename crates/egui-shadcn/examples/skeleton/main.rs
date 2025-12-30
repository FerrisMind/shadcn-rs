#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::CentralPanel;
use egui_shadcn::{
    skeleton, skeleton_text, SkeletonProps, Theme,
};

struct SkeletonExample {
    theme: Theme,
}

impl SkeletonExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for SkeletonExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Skeleton Component");
            ui.add_space(16.0);

            ui.label("Text loading placeholder:");
            ui.add_space(8.0);
            skeleton_text(ui, &self.theme, 3, 16.0);

            ui.add_space(24.0);
            ui.label("Card-like layout:");
            ui.horizontal(|ui| {
                // Avatar skeleton (circle)
                skeleton(ui, &self.theme, SkeletonProps::new().width(48.0).height(48.0).circle(true));
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    skeleton(ui, &self.theme, SkeletonProps::new().width(200.0).height(16.0));
                    ui.add_space(8.0);
                    skeleton(ui, &self.theme, SkeletonProps::new().width(150.0).height(14.0));
                });
            });

            ui.add_space(24.0);
            ui.label("Image placeholder:");
            skeleton(ui, &self.theme, SkeletonProps::new().width(300.0).height(180.0));

            ui.add_space(24.0);
            ui.label("Button placeholder:");
            skeleton(ui, &self.theme, SkeletonProps::new().width(120.0).height(36.0));

            ui.add_space(24.0);
            ui.label("Input field placeholder:");
            skeleton(ui, &self.theme, SkeletonProps::new().height(40.0));
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Skeleton example",
        options,
        Box::new(|_cc| Ok(Box::new(SkeletonExample::new()))),
    )
}
