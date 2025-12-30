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
    avatar, AvatarProps, AvatarSize, AvatarVariant, Theme,
};

struct AvatarExample {
    theme: Theme,
}

impl AvatarExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for AvatarExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Avatar Component");
            ui.add_space(16.0);

            ui.label("Sizes (1-9):");
            ui.horizontal(|ui| {
                avatar(ui, &self.theme, AvatarProps::new("S1").size(AvatarSize::Size1));
                avatar(ui, &self.theme, AvatarProps::new("S2").size(AvatarSize::Size2));
                avatar(ui, &self.theme, AvatarProps::new("S3").size(AvatarSize::Size3));
                avatar(ui, &self.theme, AvatarProps::new("S4").size(AvatarSize::Size4));
                avatar(ui, &self.theme, AvatarProps::new("S5").size(AvatarSize::Size5));
                avatar(ui, &self.theme, AvatarProps::new("S6").size(AvatarSize::Size6));
                avatar(ui, &self.theme, AvatarProps::new("S7").size(AvatarSize::Size7));
                avatar(ui, &self.theme, AvatarProps::new("S8").size(AvatarSize::Size8));
                avatar(ui, &self.theme, AvatarProps::new("S9").size(AvatarSize::Size9));
            });

            ui.add_space(16.0);
            ui.label("Soft variant (default):");
            ui.horizontal(|ui| {
                avatar(ui, &self.theme, AvatarProps::new("JD").size(AvatarSize::Size5));
                avatar(ui, &self.theme, AvatarProps::new("AB").size(AvatarSize::Size5)
                    .color(Color32::from_rgb(239, 68, 68)));
                avatar(ui, &self.theme, AvatarProps::new("CD").size(AvatarSize::Size5)
                    .color(Color32::from_rgb(34, 197, 94)));
                avatar(ui, &self.theme, AvatarProps::new("EF").size(AvatarSize::Size5)
                    .color(Color32::from_rgb(59, 130, 246)));
            });

            ui.add_space(16.0);
            ui.label("Solid variant:");
            ui.horizontal(|ui| {
                avatar(ui, &self.theme, AvatarProps::new("JD").size(AvatarSize::Size5)
                    .variant(AvatarVariant::Solid));
                avatar(ui, &self.theme, AvatarProps::new("AB").size(AvatarSize::Size5)
                    .variant(AvatarVariant::Solid)
                    .color(Color32::from_rgb(239, 68, 68)));
                avatar(ui, &self.theme, AvatarProps::new("CD").size(AvatarSize::Size5)
                    .variant(AvatarVariant::Solid)
                    .color(Color32::from_rgb(34, 197, 94)));
                avatar(ui, &self.theme, AvatarProps::new("EF").size(AvatarSize::Size5)
                    .variant(AvatarVariant::Solid)
                    .color(Color32::from_rgb(59, 130, 246)));
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Avatar example",
        options,
        Box::new(|_cc| Ok(Box::new(AvatarExample::new()))),
    )
}
