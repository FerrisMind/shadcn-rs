#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui_shadcn::{SeparatorOrientation, SeparatorProps, Theme, separator};

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
        egui::CentralPanel::default().show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            let center = panel_rect.center();
            let max_width = panel_rect.width().min(260.0);

            egui::Area::new("separator_demo_center".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .fixed_pos(center)
                .show(ui.ctx(), |ui| {
                    ui.set_max_width(max_width);

                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::new(0.0, 4.0);
                        ui.label(egui::RichText::new("Radix Primitives").size(14.0));
                        ui.label(
                            egui::RichText::new("An open-source UI component library.")
                                .size(14.0)
                                .color(self.theme.palette.muted_foreground),
                        );
                    });

                    ui.add_space(16.0);

                    separator(ui, &self.theme, SeparatorProps::default());

                    ui.add_space(16.0);

                    ui.horizontal(|row| {
                        row.set_min_height(20.0);
                        row.spacing_mut().item_spacing =
                            egui::Vec2::new(16.0, row.spacing().item_spacing.y);

                        row.label(egui::RichText::new("Blog").size(14.0));
                        separator(
                            row,
                            &self.theme,
                            SeparatorProps {
                                orientation: SeparatorOrientation::Vertical,
                                length: Some(20.0),
                                ..SeparatorProps::default()
                            },
                        );
                        row.label(egui::RichText::new("Docs").size(14.0));
                        separator(
                            row,
                            &self.theme,
                            SeparatorProps {
                                orientation: SeparatorOrientation::Vertical,
                                length: Some(20.0),
                                ..SeparatorProps::default()
                            },
                        );
                        row.label(egui::RichText::new("Source").size(14.0));
                    });
                });
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
