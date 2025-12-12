//! Пример Checkbox, повторяющий shadcn/ui `checkbox-demo`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, ControlVariant, Label, Theme, checkbox};

struct CheckboxDemo {
    theme: Theme,
    terms: bool,
    terms_2: bool,
    notifications: bool,
    notifications_2: bool,
}

impl CheckboxDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            terms: false,
            terms_2: true,
            notifications: false,
            notifications_2: true,
        }
    }
}

impl App for CheckboxDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 24.0;

                // Row 1
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.terms,
                        "Accept terms and conditions",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });

                // Row 2
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.terms_2,
                        "",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                    row.vertical(|col| {
                        col.spacing_mut().item_spacing.y = 8.0;
                        Label::new("Accept terms and conditions")
                            .size(ControlSize::Sm)
                            .show(col, &self.theme);
                        col.label(
                            egui::RichText::new(
                                "By clicking this checkbox, you agree to the terms and conditions.",
                            )
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                        );
                    });
                });

                // Row 3
                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 12.0;
                    let _ = checkbox(
                        row,
                        &self.theme,
                        &mut self.notifications,
                        "Enable notifications",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        false,
                    );
                });

                // Row 4 (clickable label container)
                let checked_border = egui::Color32::from_rgb(37, 99, 235); // blue-600
                let checked_bg = egui::Color32::from_rgba_unmultiplied(37, 99, 235, 20); // subtle blue bg
                let border_color = if self.notifications_2 {
                    checked_border
                } else {
                    egui::Color32::from_gray(80)
                };
                let fill_color = if self.notifications_2 {
                    checked_bg
                } else {
                    egui::Color32::TRANSPARENT
                };

                let mut checkbox_clicked = false;
                let frame_response = egui::Frame::NONE
                    .fill(fill_color)
                    .stroke(egui::Stroke::new(1.0, border_color))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |frame_ui| {
                        frame_ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 12.0;
                            let resp = checkbox(
                                row,
                                &self.theme,
                                &mut self.notifications_2,
                                "",
                                ControlVariant::Primary,
                                ControlSize::Md,
                                true,
                            );
                            checkbox_clicked = resp.clicked();
                            row.vertical(|col| {
                                col.spacing_mut().item_spacing.y = 6.0;
                                col.label(
                                    egui::RichText::new("Enable notifications")
                                        .size(14.0)
                                        .strong(),
                                );
                                col.label(
                                    egui::RichText::new(
                                        "You can enable or disable notifications at any time.",
                                    )
                                    .color(self.theme.palette.muted_foreground)
                                    .size(12.0),
                                );
                            });
                        });
                    });

                if frame_response.response.clicked() && !checkbox_clicked {
                    self.notifications_2 = !self.notifications_2;
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Checkbox example",
        options,
        Box::new(|_cc| Ok(Box::new(CheckboxDemo::new()))),
    )
}
