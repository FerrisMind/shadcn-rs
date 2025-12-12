//! Пример Tooltip, повторяющий shadcn/ui `tooltip-demo`.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{App, Frame, NativeOptions, egui};
use egui_shadcn::{ControlSize, ControlVariant, Theme, TooltipProps, button, tooltip};

struct TooltipDemo {
    theme: Theme,
}

impl TooltipDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for TooltipDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = button(
                ui,
                &self.theme,
                "Hover",
                ControlVariant::Outline,
                ControlSize::Md,
                true,
            );
            let _ = tooltip(
                &response,
                ui,
                &self.theme,
                TooltipProps::new("Add to library"),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "Tooltip example",
        options,
        Box::new(|_cc| Ok(Box::new(TooltipDemo::new()))),
    )
}
