#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::Color32;
use egui_shadcn::{SeparatorOrientation, SeparatorProps, SeparatorSize, Theme, separator};

struct SeparatorDemo {
    theme: Theme,
}

impl SeparatorDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for SeparatorDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Separator Component");
            ui.add_space(16.0);

            // -- Basic horizontal --
            ui.label(egui::RichText::new("Basic horizontal").strong());
            ui.add_space(4.0);
            ui.vertical(|ui| {
                ui.label("Radix Primitives");
                ui.label(
                    egui::RichText::new("An open-source UI component library.")
                        .color(self.theme.palette.muted_foreground),
                );
                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);
                ui.horizontal(|row| {
                    row.label("Blog");
                    separator(
                        row,
                        &self.theme,
                        SeparatorProps::default()
                            .orientation(SeparatorOrientation::Vertical)
                            .length(20.0),
                    );
                    row.label("Docs");
                    separator(
                        row,
                        &self.theme,
                        SeparatorProps::default()
                            .orientation(SeparatorOrientation::Vertical)
                            .length(20.0),
                    );
                    row.label("Source");
                });
            });

            ui.add_space(16.0);

            // -- Sizes --
            ui.label(egui::RichText::new("Sizes").strong());
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().size(SeparatorSize::Size1),
            );
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().size(SeparatorSize::Size2),
            );
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().size(SeparatorSize::Size3),
            );
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().size(SeparatorSize::Size4),
            );

            ui.add_space(16.0);

            // -- Thickness --
            ui.label(egui::RichText::new("Thickness").strong());
            ui.add_space(4.0);
            separator(ui, &self.theme, SeparatorProps::default().thickness(1.0));
            ui.add_space(4.0);
            separator(ui, &self.theme, SeparatorProps::default().thickness(2.0));
            ui.add_space(4.0);
            separator(ui, &self.theme, SeparatorProps::default().thickness(4.0));

            ui.add_space(16.0);

            // -- Custom Color --
            ui.label(egui::RichText::new("Custom Color").strong());
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().color(Color32::from_rgb(239, 68, 68)),
            );

            ui.add_space(16.0);

            // -- High Contrast --
            ui.label(egui::RichText::new("High Contrast").strong());
            ui.add_space(4.0);
            separator(ui, &self.theme, SeparatorProps::default());
            ui.add_space(4.0);
            separator(
                ui,
                &self.theme,
                SeparatorProps::default().high_contrast(true),
            );

            ui.add_space(16.0);

            // -- With Gap --
            ui.label(egui::RichText::new("With Gap (8px)").strong());
            ui.add_space(4.0);
            ui.label("Content above");
            separator(ui, &self.theme, SeparatorProps::default().gap(8.0));
            ui.label("Content below");
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Separator example",
        options,
        Box::new(|_cc| Ok(Box::new(SeparatorDemo::new()))),
    )
}
