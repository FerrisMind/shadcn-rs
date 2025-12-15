#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
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
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(80.0);

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
                    TooltipProps::new("Add to library")
                        .delay_ms(0)
                        .skip_delay_ms(0)
                        .show_arrow(true)
                        .side_offset(8.0),
                );
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Tooltip example",
        options,
        Box::new(|_cc| Ok(Box::new(TooltipDemo::new()))),
    )
}
