#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui::CentralPanel;
use egui_shadcn::{AlertProps, AlertVariant, Theme, alert};

struct AlertExample {
    theme: Theme,
}

impl AlertExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}

impl App for AlertExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alert Component");
            ui.add_space(16.0);

            ui.label("Default Alert:");
            alert(
                ui,
                &self.theme,
                AlertProps::new("This is a default alert message."),
            );
            ui.add_space(12.0);

            ui.label("Info Alert:");
            alert(
                ui,
                &self.theme,
                AlertProps::new("Heads up! This is an informational message.")
                    .variant(AlertVariant::Info)
                    .title("Info"),
            );
            ui.add_space(12.0);

            ui.label("Success Alert:");
            alert(
                ui,
                &self.theme,
                AlertProps::new("Your changes have been saved successfully.")
                    .variant(AlertVariant::Success)
                    .title("Success"),
            );
            ui.add_space(12.0);

            ui.label("Warning Alert:");
            alert(
                ui,
                &self.theme,
                AlertProps::new("Please review your settings before continuing.")
                    .variant(AlertVariant::Warning)
                    .title("Warning"),
            );
            ui.add_space(12.0);

            ui.label("Destructive Alert:");
            alert(
                ui,
                &self.theme,
                AlertProps::new("This action cannot be undone. Data will be permanently deleted.")
                    .variant(AlertVariant::Destructive)
                    .title("Error"),
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Alert example",
        options,
        Box::new(|_cc| Ok(Box::new(AlertExample::new()))),
    )
}
