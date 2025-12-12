//! Пример RadioGroup, содержащий `radio-group-demo` и `radio-group-form`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::radio::{RadioGroup, RadioOption};
use egui_shadcn::{ControlSize, ControlVariant, SeparatorProps, Theme, button, separator};

struct RadioDemo {
    theme: Theme,
    value: String,
    form_value: String,
}

impl RadioDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            value: "comfortable".to_string(),
            form_value: "all".to_string(),
        }
    }
}

impl App for RadioDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;

                // radio-group-demo
                let options = vec![
                    RadioOption::new("default".to_string(), "Default"),
                    RadioOption::new("comfortable".to_string(), "Comfortable"),
                    RadioOption::new("compact".to_string(), "Compact"),
                ];
                RadioGroup::new("radio-group-demo", &mut self.value, &options)
                    .show(ui, &self.theme);

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                // radio-group-form (упрощённо)
                ui.label(
                    egui::RichText::new("Notify me about...")
                        .text_style(egui::TextStyle::Button)
                        .size(14.0)
                        .strong(),
                );
                let form_options = vec![
                    RadioOption::new("all".to_string(), "All new messages"),
                    RadioOption::new("mentions".to_string(), "Direct messages and mentions"),
                    RadioOption::new("none".to_string(), "Nothing"),
                ];
                RadioGroup::new("radio-group-form", &mut self.form_value, &form_options)
                    .show(ui, &self.theme);

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
    let options = NativeOptions::default();
    eframe::run_native(
        "RadioGroup example",
        options,
        Box::new(|_cc| Ok(Box::new(RadioDemo::new()))),
    )
}
