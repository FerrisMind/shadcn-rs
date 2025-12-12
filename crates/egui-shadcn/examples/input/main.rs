#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{App, Frame, egui};
use egui_shadcn::{
    ControlSize, ControlVariant, Input, InputSize, InputType, Label, SeparatorProps, Theme, button,
    separator,
};

struct InputDemo {
    theme: Theme,
    email: String,
    disabled_email: String,
    picture_path: String,
    subscribe_email: String,
    labeled_email: String,
    text_email: String,
    username: String,
}

impl InputDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            email: String::new(),
            disabled_email: String::new(),
            picture_path: String::new(),
            subscribe_email: String::new(),
            labeled_email: String::new(),
            text_email: String::new(),
            username: String::new(),
        }
    }
}

impl App for InputDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 16.0;
                ui.set_max_width(360.0);

                let input_id = ui.make_persistent_id("input-email");
                Input::new(input_id)
                    .input_type(InputType::Email)
                    .placeholder("Email")
                    .size(InputSize::Size2)
                    .width(ui.available_width())
                    .show(ui, &self.theme, &mut self.email);

                let disabled_id = ui.make_persistent_id("input-disabled");
                Input::new(disabled_id)
                    .input_type(InputType::Email)
                    .placeholder("Email")
                    .size(InputSize::Size2)
                    .enabled(false)
                    .width(ui.available_width())
                    .show(ui, &self.theme, &mut self.disabled_email);

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let picture_id = group.make_persistent_id("input-picture");
                    Label::new("Picture")
                        .for_id(picture_id)
                        .size(ControlSize::Sm)
                        .show(group, &self.theme);
                    Input::new(picture_id)
                        .size(InputSize::Size2)
                        .width(group.available_width())
                        .show(group, &self.theme, &mut self.picture_path);
                });

                ui.horizontal(|row| {
                    row.spacing_mut().item_spacing.x = 8.0;
                    let email_id = row.make_persistent_id("input-subscribe");
                    Input::new(email_id)
                        .input_type(InputType::Email)
                        .placeholder("Email")
                        .size(InputSize::Size2)
                        .width(row.available_width() - 100.0)
                        .show(row, &self.theme, &mut self.subscribe_email);
                    let _ = button(
                        row,
                        &self.theme,
                        "Subscribe",
                        ControlVariant::Outline,
                        ControlSize::Md,
                        true,
                    );
                });

                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let email_id = group.make_persistent_id("input-with-label");
                    Label::new("Email")
                        .for_id(email_id)
                        .size(ControlSize::Sm)
                        .show(group, &self.theme);
                    Input::new(email_id)
                        .input_type(InputType::Email)
                        .placeholder("Email")
                        .size(InputSize::Size2)
                        .width(group.available_width())
                        .show(group, &self.theme, &mut self.labeled_email);
                });

                ui.vertical(|group| {
                    group.spacing_mut().item_spacing.y = 8.0;
                    let email_id = group.make_persistent_id("input-with-text");
                    Label::new("Email")
                        .for_id(email_id)
                        .size(ControlSize::Sm)
                        .show(group, &self.theme);
                    Input::new(email_id)
                        .input_type(InputType::Email)
                        .placeholder("Email")
                        .size(InputSize::Size2)
                        .width(group.available_width())
                        .show(group, &self.theme, &mut self.text_email);
                    group.label(
                        egui::RichText::new("Enter your email address.")
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                    );
                });

                ui.add_space(8.0);
                separator(ui, &self.theme, SeparatorProps::default());
                ui.add_space(8.0);

                ui.vertical(|form| {
                    form.spacing_mut().item_spacing.y = 8.0;
                    let username_id = form.make_persistent_id("username");
                    Label::new("Username")
                        .for_id(username_id)
                        .size(ControlSize::Sm)
                        .show(form, &self.theme);
                    Input::new(username_id)
                        .placeholder("shadcn")
                        .size(InputSize::Size2)
                        .width(form.available_width())
                        .show(form, &self.theme, &mut self.username);
                    form.label(
                        egui::RichText::new("This is your public display name.")
                            .color(self.theme.palette.muted_foreground)
                            .size(12.0),
                    );
                    let _ = button(
                        form,
                        &self.theme,
                        "Submit",
                        ControlVariant::Primary,
                        ControlSize::Md,
                        true,
                    );
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Input example",
        options,
        Box::new(|_cc| Ok(Box::new(InputDemo::new()))),
    )
}
