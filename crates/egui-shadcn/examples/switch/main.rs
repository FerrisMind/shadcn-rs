#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui_shadcn::{ControlSize, ControlVariant, Theme, button, switch};

struct SwitchDemo {
    theme: Theme,
    airplane_mode: bool,
    marketing_emails: bool,
    security_emails: bool,
}

impl SwitchDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            airplane_mode: false,
            marketing_emails: false,
            security_emails: true,
        }
    }
}

impl App for SwitchDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;
                ui.set_max_width(420.0);

                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 8.0;
                    let _ = switch(
                        row,
                        &self.theme,
                        &mut self.airplane_mode,
                        "Airplane Mode",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("Email Notifications")
                        .text_style(egui::TextStyle::Button)
                        .size(16.0)
                        .strong(),
                );

                ui.vertical(|list| {
                    list.spacing_mut().item_spacing.y = 8.0;

                    for (id_suffix, title, description, value, enabled) in [
                        (
                            "marketing",
                            "Marketing emails",
                            "Receive emails about new products, features, and more.",
                            &mut self.marketing_emails,
                            true,
                        ),
                        (
                            "security",
                            "Security emails",
                            "Receive emails about your account security.",
                            &mut self.security_emails,
                            false,
                        ),
                    ] {
                        egui::Frame::NONE
                            .fill(self.theme.palette.background)
                            .stroke(egui::Stroke::new(1.0, self.theme.palette.border))
                            .corner_radius(egui::CornerRadius::same(8))
                            .inner_margin(egui::Margin::symmetric(12, 10))
                            .show(list, |item_ui| {
                                item_ui.push_id(id_suffix, |item_ui| {
                                    item_ui.horizontal(|row| {
                                        row.set_width(row.available_width());
                                        row.vertical(|text| {
                                            text.spacing_mut().item_spacing.y = 4.0;
                                            text.label(
                                                egui::RichText::new(title).size(14.0).strong(),
                                            );
                                            text.label(
                                                egui::RichText::new(description)
                                                    .color(self.theme.palette.muted_foreground)
                                                    .size(12.0),
                                            );
                                        });
                                        row.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |right| {
                                                let _ = switch(
                                                    right,
                                                    &self.theme,
                                                    value,
                                                    "",
                                                    ControlVariant::Primary,
                                                    ControlSize::Sm,
                                                    enabled,
                                                );
                                            },
                                        );
                                    });
                                });
                            });
                    }
                });

                let _ = button(
                    ui,
                    &self.theme,
                    "Submit",
                    ControlVariant::Primary,
                    ControlSize::Md,
                    true,
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Switch example",
        options,
        Box::new(|_cc| Ok(Box::new(SwitchDemo::new()))),
    )
}
