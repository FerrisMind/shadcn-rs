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
use egui_shadcn::{
    AlertDialogProps, AlertDialogResult, Button, ButtonVariant, Theme, alert_dialog,
};

struct AlertDialogExample {
    theme: Theme,
    show_delete_dialog: bool,
    show_confirm_dialog: bool,
    last_result: String,
}

impl AlertDialogExample {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            show_delete_dialog: false,
            show_confirm_dialog: false,
            last_result: String::new(),
        }
    }
}

impl App for AlertDialogExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alert Dialog Component");
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                if Button::new("Delete Item")
                    .variant(ButtonVariant::Destructive)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.show_delete_dialog = true;
                }

                if Button::new("Confirm Action")
                    .variant(ButtonVariant::Default)
                    .show(ui, &self.theme)
                    .clicked()
                {
                    self.show_confirm_dialog = true;
                }
            });

            ui.add_space(16.0);
            if !self.last_result.is_empty() {
                ui.label(format!("Last result: {}", self.last_result));
            }

            // Delete confirmation dialog
            let result = alert_dialog(
                ui,
                &self.theme,
                AlertDialogProps::new(
                    &mut self.show_delete_dialog,
                    "Are you absolutely sure?",
                    "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
                )
                .destructive()
                .cancel_text("Cancel")
                .action_text("Yes, delete account"),
            );

            match result {
                AlertDialogResult::Confirmed => self.last_result = "Delete confirmed!".to_string(),
                AlertDialogResult::Cancelled => self.last_result = "Delete cancelled.".to_string(),
                AlertDialogResult::None => {}
            }

            // Standard confirmation dialog
            let result = alert_dialog(
                ui,
                &self.theme,
                AlertDialogProps::new(
                    &mut self.show_confirm_dialog,
                    "Confirm your action",
                    "Are you sure you want to proceed with this action?",
                )
                .cancel_text("No, go back")
                .action_text("Yes, continue"),
            );

            match result {
                AlertDialogResult::Confirmed => self.last_result = "Action confirmed!".to_string(),
                AlertDialogResult::Cancelled => self.last_result = "Action cancelled.".to_string(),
                AlertDialogResult::None => {}
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Alert Dialog example",
        options,
        Box::new(|_cc| Ok(Box::new(AlertDialogExample::new()))),
    )
}
