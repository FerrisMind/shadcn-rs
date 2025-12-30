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
use egui_shadcn::{
    badge, BadgeProps, BadgeSize, BadgeVariant, Theme,
};

struct BadgeExample {
    theme: Theme,
}

impl BadgeExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for BadgeExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Badge Component");
            ui.add_space(16.0);

            ui.label("Sizes:");
            ui.horizontal(|ui| {
                badge(ui, &self.theme, BadgeProps::new("Size 1").size(BadgeSize::Size1));
                badge(ui, &self.theme, BadgeProps::new("Size 2").size(BadgeSize::Size2));
                badge(ui, &self.theme, BadgeProps::new("Size 3").size(BadgeSize::Size3));
            });

            ui.add_space(16.0);
            ui.label("Variants:");
            ui.horizontal(|ui| {
                badge(ui, &self.theme, BadgeProps::new("Solid").variant(BadgeVariant::Solid));
                badge(ui, &self.theme, BadgeProps::new("Soft").variant(BadgeVariant::Soft));
                badge(ui, &self.theme, BadgeProps::new("Surface").variant(BadgeVariant::Surface));
                badge(ui, &self.theme, BadgeProps::new("Outline").variant(BadgeVariant::Outline));
            });

            ui.add_space(16.0);
            ui.label("Custom colors:");
            ui.horizontal(|ui| {
                badge(ui, &self.theme, BadgeProps::new("Error")
                    .variant(BadgeVariant::Solid)
                    .color(Color32::from_rgb(239, 68, 68)));
                badge(ui, &self.theme, BadgeProps::new("Success")
                    .variant(BadgeVariant::Solid)
                    .color(Color32::from_rgb(34, 197, 94)));
                badge(ui, &self.theme, BadgeProps::new("Warning")
                    .variant(BadgeVariant::Solid)
                    .color(Color32::from_rgb(245, 158, 11)));
                badge(ui, &self.theme, BadgeProps::new("Info")
                    .variant(BadgeVariant::Solid)
                    .color(Color32::from_rgb(59, 130, 246)));
            });

            ui.add_space(16.0);
            ui.label("Soft variants with colors:");
            ui.horizontal(|ui| {
                badge(ui, &self.theme, BadgeProps::new("New")
                    .color(Color32::from_rgb(168, 85, 247)));
                badge(ui, &self.theme, BadgeProps::new("Beta")
                    .color(Color32::from_rgb(236, 72, 153)));
                badge(ui, &self.theme, BadgeProps::new("Pro")
                    .color(Color32::from_rgb(14, 165, 233)));
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Badge example",
        options,
        Box::new(|_cc| Ok(Box::new(BadgeExample::new()))),
    )
}
