//! Пример Label, повторяющий shadcn/ui `label-demo`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, ControlVariant, Theme, checkbox};

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

impl App for LabelDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|row| {
                row.spacing_mut().item_spacing.x = 8.0;
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
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Label example",
        options,
        Box::new(|_cc| Ok(Box::new(LabelDemo::new()))),
    )
}
