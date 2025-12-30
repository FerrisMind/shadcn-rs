#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[path = "../_shared/icon.rs"]
mod icon;
#[path = "../_shared/screenshot.rs"]
mod screenshot;

use eframe::{App, Frame, egui};
use egui_shadcn::{
    ControlSize, ControlVariant, Label, Textarea, TextareaResize, TextareaSize, Theme, button,
};

struct TextareaDemo {
    theme: Theme,
    message_demo: String,
    message_disabled: String,
    message_with_label: String,
    message_with_text: String,
    message_with_button: String,
    bio_text: String,
}

impl TextareaDemo {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
            message_demo: String::new(),
            message_disabled: String::new(),
            message_with_label: String::new(),
            message_with_text: String::new(),
            message_with_button: String::new(),
            bio_text: String::new(),
        }
    }
}

fn example_card(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new(title).strong());
        ui.add_space(6.0);
        content(ui);
    });
}

impl App for TextareaDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        screenshot::apply_screenshot_scale(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
            ui.heading("Textarea");
            ui.add_space(12.0);

            egui::Grid::new("textarea_examples_grid")
                .num_columns(3)
                .spacing(egui::vec2(24.0, 18.0))
                .show(ui, |grid| {
                    let area_width = 320.0;

                    example_card(grid, "Textarea", |ui| {
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        Textarea::new("textarea-demo")
                            .placeholder("Type your message here.")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::Both)
                            .show(ui, &self.theme, &mut self.message_demo);
                    });

                    example_card(grid, "Disabled", |ui| {
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        Textarea::new("textarea-disabled")
                            .placeholder("Type your message here.")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::Both)
                            .enabled(false)
                            .show(ui, &self.theme, &mut self.message_disabled);
                    });

                    example_card(grid, "With label", |ui| {
                        ui.spacing_mut().item_spacing.y = 12.0;
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        let message_id = ui.make_persistent_id("textarea-with-label");
                        Label::new("Your message")
                            .for_id(message_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Textarea::new(message_id)
                            .placeholder("Type your message here.")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::Both)
                            .show(ui, &self.theme, &mut self.message_with_label);
                        ui.add_space(16.0);
                    });
                    grid.end_row();

                    example_card(grid, "With helper text", |ui| {
                        ui.spacing_mut().item_spacing.y = 12.0;
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        let message_id = ui.make_persistent_id("textarea-with-text");
                        Label::new("Your Message")
                            .for_id(message_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Textarea::new(message_id)
                            .placeholder("Type your message here.")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::Both)
                            .show(ui, &self.theme, &mut self.message_with_text);
                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new("Your message will be copied to the support team.")
                                .color(self.theme.palette.muted_foreground)
                                .size(12.0),
                        );
                    });

                    example_card(grid, "With button", |ui| {
                        ui.spacing_mut().item_spacing.y = 8.0;
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        let message_id = ui.make_persistent_id("textarea-with-button");
                        Textarea::new(message_id)
                            .placeholder("Type your message here.")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::Both)
                            .show(ui, &self.theme, &mut self.message_with_button);
                        ui.add_space(12.0);
                        let _ = button(
                            ui,
                            &self.theme,
                            "Send message",
                            ControlVariant::Primary,
                            ControlSize::Md,
                            true,
                        );
                    });

                    example_card(grid, "Form", |ui| {
                        ui.spacing_mut().item_spacing.y = 12.0;
                        ui.set_min_width(area_width);
                        ui.set_max_width(area_width);
                        let bio_id = ui.make_persistent_id("textarea-bio");
                        Label::new("Bio")
                            .for_id(bio_id)
                            .size(ControlSize::Sm)
                            .show(ui, &self.theme);
                        Textarea::new(bio_id)
                            .placeholder("Tell us a little bit about yourself")
                            .size(TextareaSize::Size2)
                            .width(area_width)
                            .resize(TextareaResize::None)
                            .show(ui, &self.theme, &mut self.bio_text);
                        ui.add_space(18.0);
                        ui.label(
                            egui::RichText::new("You can @mention other users and organizations.")
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
                    grid.end_row();
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = icon::native_options();
    eframe::run_native(
        "Textarea example",
        options,
        Box::new(|_cc| Ok(Box::new(TextareaDemo::new()))),
    )
}
