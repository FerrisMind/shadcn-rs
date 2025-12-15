#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::RichText;
use egui_shadcn::{ControlSize, ControlVariant, Label, Theme, checkbox};

struct LabelDemo {
    theme: Theme,
    terms: bool,
}

impl LabelDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            terms: false,
        }
    }
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

impl App for LabelDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            let avail = ui.available_size();
            ui.allocate_ui_with_layout(avail, egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Label");
                ui.add_space(12.0);

                let card_width = 320.0;
                ui.vertical_centered(|ui| {
                    ui.set_min_width(card_width);
                    ui.set_max_width(card_width);
                    example_card(ui, "Label", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            let _ = checkbox(
                                row,
                                &self.theme,
                                &mut self.terms,
                                "",
                                ControlVariant::Primary,
                                ControlSize::Md,
                                true,
                            );
                            let label_resp = Label::new("Accept terms and conditions")
                                .size(ControlSize::Md)
                                .show(row, &self.theme);
                            if label_resp.clicked() {
                                self.terms = !self.terms;
                            }
                        });
                    });
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Label example",
        options,
        Box::new(|_cc| Ok(Box::new(LabelDemo::new()))),
    )
}
