#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;

use eframe::{egui, App, Frame};
use egui::RichText;
use egui_shadcn::{
    button, ControlSize, ControlVariant, Input, InputSize, InputType, Label, Theme,
};
use rfd::FileDialog;

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

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

impl App for InputDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Input");
            ui.add_space(12.0);

            egui::Grid::new("input_examples_grid")
                .num_columns(2)
                .spacing(egui::vec2(24.0, 18.0))
                .show(ui, |grid| {
                    let card_width = 320.0;

                    let basic_id = grid.make_persistent_id("input-email");
                    example_card(grid, "Input", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        Input::new(basic_id)
                            .input_type(InputType::Email)
                            .placeholder("Email")
                            .size(InputSize::Size2)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.email);
                    });

                    let disabled_id = grid.make_persistent_id("input-disabled");
                    example_card(grid, "Disabled", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        Input::new(disabled_id)
                            .input_type(InputType::Email)
                            .placeholder("Email")
                            .size(InputSize::Size2)
                            .enabled(false)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.disabled_email);
                    });
                    grid.end_row();

                    let picture_id = grid.make_persistent_id("input-picture");
                    example_card(grid, "File", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.spacing_mut().item_spacing.y = 8.0;
                        Label::new("Picture")
                            .for_id(picture_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        let resp = Input::new(picture_id)
                            .placeholder("No file selected")
                            .size(InputSize::Size2)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.picture_path);
                        if resp.clicked() {
                            if let Some(path) = FileDialog::new().pick_file() {
                                self.picture_path = path.display().to_string();
                            }
                        }
                    });

                    let subscribe_id = grid.make_persistent_id("input-subscribe");
                    example_card(grid, "With button", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.horizontal(|row| {
                            row.spacing_mut().item_spacing.x = 8.0;
                            Input::new(subscribe_id)
                                .input_type(InputType::Email)
                                .placeholder("Email")
                                .size(InputSize::Size2)
                                .width(card_width - 118.0)
                                .show(row, &self.theme, &mut self.subscribe_email);
                            row.add_space(8.0);
                            let _ = button(
                                row,
                                &self.theme,
                                "Subscribe",
                                ControlVariant::Outline,
                                ControlSize::Sm,
                                true,
                            );
                        });
                    });
                    grid.end_row();

                    let labeled_id = grid.make_persistent_id("input-with-label");
                    example_card(grid, "With label", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.spacing_mut().item_spacing.y = 8.0;
                        Label::new("Email")
                            .for_id(labeled_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Input::new(labeled_id)
                            .input_type(InputType::Email)
                            .placeholder("Email")
                            .size(InputSize::Size2)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.labeled_email);
                    });

                    let text_id = grid.make_persistent_id("input-with-text");
                    example_card(grid, "With text", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.spacing_mut().item_spacing.y = 8.0;
                        Label::new("Email")
                            .for_id(text_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Input::new(text_id)
                            .input_type(InputType::Email)
                            .placeholder("Email")
                            .size(InputSize::Size2)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.text_email);
                        ui.label(
                            RichText::new("Enter your email address.")
                                .color(self.theme.palette.muted_foreground)
                                .size(12.0),
                        );
                    });
                    grid.end_row();

                    let username_id = grid.make_persistent_id("username");
                    example_card(grid, "Form", |ui| {
                        ui.set_min_width(card_width);
                        ui.set_max_width(card_width);
                        ui.spacing_mut().item_spacing.y = 10.0;
                        Label::new("Username")
                            .for_id(username_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Input::new(username_id)
                            .placeholder("shadcn")
                            .size(InputSize::Size2)
                            .width(card_width)
                            .show(ui, &self.theme, &mut self.username);
                        ui.label(
                            RichText::new("This is your public display name.")
                                .color(self.theme.palette.muted_foreground)
                                .size(12.0),
                        );
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
